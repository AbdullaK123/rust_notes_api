#!/bin/bash
set -e  # Exit on any error

# =============================================================================
# CONFIGURATION
# =============================================================================
readonly DB_NAME="notes_db_dev"
readonly DB_USER="postgres"
readonly DB_PASSWORD="password123"
readonly DB_PORT="5432"
readonly CONTAINER_NAME="notes-postgres"
readonly POSTGRES_VERSION="17"
readonly MAX_WAIT_ATTEMPTS=30
readonly WAIT_SLEEP_INTERVAL=1

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly NC='\033[0m' # No Color

# =============================================================================
# UTILITY FUNCTIONS
# =============================================================================
log_info() {
    echo -e "${YELLOW}$1${NC}"
}

log_success() {
    echo -e "${GREEN}$1${NC}"
}

log_error() {
    echo -e "${RED}$1${NC}"
}

# =============================================================================
# CLEANUP FUNCTIONS
# =============================================================================
kill_port_processes() {
    log_info "Checking for processes on port $DB_PORT..."
    if lsof -Pi :$DB_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
        log_error "Found process running on port $DB_PORT. Killing it..."
        # shellcheck disable=SC2046
        sudo kill -9 $(lsof -Pi :$DB_PORT -sTCP:LISTEN -t) 2>/dev/null || true
        sleep 2
    fi
}

cleanup_existing_container() {
    log_info "Removing existing container if it exists..."
    docker stop $CONTAINER_NAME 2>/dev/null || true
    docker rm $CONTAINER_NAME 2>/dev/null || true
}

# =============================================================================
# DATABASE SETUP FUNCTIONS
# =============================================================================
start_postgres_container() {
    log_info "Starting PostgreSQL $POSTGRES_VERSION container..."
    docker run -d \
        --name $CONTAINER_NAME \
        -e POSTGRES_USER=$DB_USER \
        -e POSTGRES_PASSWORD=$DB_PASSWORD \
        -e POSTGRES_DB=postgres \
        -p $DB_PORT:5432 \
        postgres:$POSTGRES_VERSION
}

wait_for_postgres() {
    log_info "Waiting for PostgreSQL to be ready..."
    sleep 5
    
    for i in $(seq 1 $MAX_WAIT_ATTEMPTS); do
        if docker exec $CONTAINER_NAME pg_isready -U $DB_USER >/dev/null 2>&1; then
            log_success "PostgreSQL is ready!"
            return 0
        fi
        
        if [ $i -eq $MAX_WAIT_ATTEMPTS ]; then
            log_error "PostgreSQL failed to start after $MAX_WAIT_ATTEMPTS seconds"
            exit 1
        fi
        
        echo "Waiting for PostgreSQL... ($i/$MAX_WAIT_ATTEMPTS)"
        sleep $WAIT_SLEEP_INTERVAL
    done
}

create_database() {
    log_info "Creating database '$DB_NAME'..."
    docker exec $CONTAINER_NAME psql -U $DB_USER -c "CREATE DATABASE $DB_NAME;" 2>/dev/null || {
        log_info "Database '$DB_NAME' might already exist, continuing..."
    }
}

# =============================================================================
# ENVIRONMENT SETUP
# =============================================================================
generate_jwt_secret() {
    openssl rand -base64 64 | tr -d '\n'
}

create_env_file() {
    log_info "Creating .env file..."
    # shellcheck disable=SC2155
    local jwt_secret=$(generate_jwt_secret)
    
    cat > .env << EOF
DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME
REDIS_URL=redis://localhost:6379
SECRET_KEY=$jwt_secret
EOF
}

# =============================================================================
# MIGRATION FUNCTIONS
# =============================================================================
check_sqlx_cli() {
    if ! command -v sqlx &> /dev/null; then
        log_error "sqlx-cli is not installed!"
        log_info "Install it with: cargo install sqlx-cli --no-default-features --features native-tls,postgres"
        return 1
    fi
    return 0
}

run_migrations() {
    log_info "Running database migrations..."
    
    if ! check_sqlx_cli; then
        log_error "Skipping migrations - sqlx-cli not available"
        return 1
    fi
    
    # Export DATABASE_URL for sqlx
    export DATABASE_URL="postgresql://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
    
    # Run migrations
    if sqlx migrate run; then
        log_success "✅ Database migrations completed successfully!"
    else
        log_error "❌ Failed to run migrations"
        return 1
    fi
}

# =============================================================================
# USER INTERACTION
# =============================================================================
show_completion_info() {
    log_success "✅ Development database setup complete!"
    log_success "Database URL: postgresql://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
    echo ""
    
    log_info "Useful commands:"
    echo "  Connect to database:    docker exec -it $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME"
    echo "  Stop database:          docker stop $CONTAINER_NAME"
    echo "  Start database:         docker start $CONTAINER_NAME"
    echo "  Remove database:        docker stop $CONTAINER_NAME && docker rm $CONTAINER_NAME"
    echo "  Run migrations:         sqlx migrate run"
    echo ""
}

prompt_psql_session() {
    read -p "Do you want to open a psql session now? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log_success "Opening psql session..."
        docker exec -it $CONTAINER_NAME psql -U $DB_USER -d $DB_NAME
    fi
}

# =============================================================================
# MAIN EXECUTION
# =============================================================================
main() {
    log_info "Setting up development database..."
    
    kill_port_processes
    cleanup_existing_container
    start_postgres_container
    wait_for_postgres
    create_database
    create_env_file
    
    # Run migrations
    if ! run_migrations; then
        log_info "💡 You can run migrations manually later with: sqlx migrate run"
    fi
    
    show_completion_info
    prompt_psql_session
}

# Execute main function
main "$@"