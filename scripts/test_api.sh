#!/bin/bash

# Configuration
BASE_URL="${API_BASE_URL:-http://localhost:8080}"
COOKIES_FILE="cookies.txt"
TEST_EMAIL="test@example.com"
TEST_PASSWORD="SecurePass123!"  # Updated to meet password requirements
TEST_USERNAME="testuser"        # Changed from full_name to username

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TESTS_PASSED=0
TESTS_FAILED=0

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to make curl request and check status
make_request() {
    local method=$1
    local endpoint=$2
    local data=$3
    local expected_status=$4
    local description=$5
    
    echo -e "\n${BLUE}Testing: ${description}${NC}"
    
    local response=$(curl -s -X $method "$BASE_URL$endpoint" \
        -H "Content-Type: application/json" \
        -b $COOKIES_FILE \
        -c $COOKIES_FILE \
        -d "$data" \
        -w "HTTPSTATUS:%{http_code}")
    
    local http_code=$(echo $response | tr -d '\n' | sed -e 's/.*HTTPSTATUS://')
    local body=$(echo $response | sed -e 's/HTTPSTATUS:.*//g')
    
    echo "Response: $body"
    
    if [ "$http_code" -eq "$expected_status" ]; then
        print_status $GREEN "✅ Status: $http_code (Expected: $expected_status)"
        ((TESTS_PASSED++))
        return 0
    else
        print_status $RED "❌ Status: $http_code (Expected: $expected_status)"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Function to make GET request with query parameters
make_get_request() {
    local endpoint=$1
    local query_params=$2
    local expected_status=$3
    local description=$4
    
    echo -e "\n${BLUE}Testing: ${description}${NC}"
    
    local url="$BASE_URL$endpoint"
    if [ ! -z "$query_params" ]; then
        url="$url?$query_params"
    fi
    
    local response=$(curl -s -G "$url" \
        -H "Content-Type: application/json" \
        -b $COOKIES_FILE \
        -c $COOKIES_FILE \
        -w "HTTPSTATUS:%{http_code}")
    
    local http_code=$(echo $response | tr -d '\n' | sed -e 's/.*HTTPSTATUS://')
    local body=$(echo $response | sed -e 's/HTTPSTATUS:.*//g')
    
    echo "Response: $body"
    
    if [ "$http_code" -eq "$expected_status" ]; then
        print_status $GREEN "✅ Status: $http_code (Expected: $expected_status)"
        ((TESTS_PASSED++))
        return 0
    else
        print_status $RED "❌ Status: $http_code (Expected: $expected_status)"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Function to create a note and return its ID
create_note() {
    local title=$1
    local content=$2
    
    local note_response=$(curl -s -X POST "$BASE_URL/notes" \
        -H "Content-Type: application/json" \
        -b $COOKIES_FILE \
        -c $COOKIES_FILE \
        -d "{\"title\":\"$title\",\"content\":\"$content\"}")
    
    echo $note_response | grep -o '"id":"[^"]*"' | cut -d'"' -f4
}

# Function to cleanup
cleanup() {
    rm -f $COOKIES_FILE
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Main test execution
main() {
    print_status $YELLOW "🧪 Testing Rust Notes API"
    print_status $BLUE "Base URL: $BASE_URL"
    
    # Clean up previous cookies
    cleanup
    
    # Test 1: Register user (with correct field names)
    make_request "POST" "/auth/register" \
        "{\"username\":\"$TEST_USERNAME\",\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}" \
        201 "User Registration"
    
    # Test 2: Login
    make_request "POST" "/auth/login" \
        "{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}" \
        200 "User Login"
    
    # Test 3: Get user profile
    make_request "GET" "/auth/me" \
        "" \
        200 "Get User Profile"
    
    # Create multiple test notes for search testing
    print_status $YELLOW "\n📝 Creating test notes for search functionality..."
    
    NOTE_ID_1=$(create_note "JavaScript Tutorial" "Learn JavaScript basics, including variables, functions, and DOM manipulation. This comprehensive guide covers ES6 features.")
    NOTE_ID_2=$(create_note "Python Data Analysis" "Using pandas and numpy for data analysis in Python. Learn to process CSV files and create visualizations.")
    NOTE_ID_3=$(create_note "Rust Web Development" "Building web applications with Rust and Actix-web framework. Perfect for system programming enthusiasts.")
    NOTE_ID_4=$(create_note "Database Design Principles" "Principles of database normalization and SQL optimization. Learn about indexing and query performance.")
    NOTE_ID_5=$(create_note "Machine Learning Basics" "Introduction to machine learning algorithms using Python libraries like scikit-learn and TensorFlow.")
    NOTE_ID_6=$(create_note "React Components" "Building reusable React components with hooks and state management. Modern frontend development techniques.")
    NOTE_ID_7=$(create_note "API Development" "RESTful API design patterns and best practices. Learn about authentication, validation, and error handling.")
    
    # Test 4: Get all notes
    make_request "GET" "/notes" \
        "" \
        200 "Get All Notes"
    
    # SEARCH FUNCTIONALITY TESTS
    print_status $YELLOW "\n🔍 Testing Search Functionality..."
    
    # Test 5: Search by title - exact match
    make_get_request "/notes" "search=JavaScript" 200 "Search by title (exact match)"
    
    # Test 6: Search by title - partial match
    make_get_request "/notes" "search=Tutorial" 200 "Search by title (partial match)"
    
    # Test 7: Search by content - single keyword
    make_get_request "/notes" "search=pandas" 200 "Search by content keyword"
    
    # Test 8: Search by content - multiple keywords
    make_get_request "/notes" "search=Python%20libraries" 200 "Search by content phrase"
    
    # Test 9: Search across title and content
    make_get_request "/notes" "search=React" 200 "Search across title and content"
    
    # Test 10: Search for technical terms
    make_get_request "/notes" "search=API" 200 "Search for technical terms"
    
    # Test 11: Case insensitive search
    make_get_request "/notes" "search=python" 200 "Case insensitive search (lowercase)"
    
    # Test 12: Case insensitive search - uppercase
    make_get_request "/notes" "search=JAVASCRIPT" 200 "Case insensitive search (uppercase)"
    
    # Test 13: Search with no results
    make_get_request "/notes" "search=nonexistentterm" 200 "Search with no results"
    
    # Test 14: Search with limit parameter
    make_get_request "/notes" "search=development&limit=2" 200 "Search with limit parameter"
    
    # Test 15: Search for programming languages
    make_get_request "/notes" "search=Rust" 200 "Search for programming language"
    
    # Test 16: Search for frameworks
    make_get_request "/notes" "search=Actix" 200 "Search for framework names"
    
    # Test 17: Empty search parameter
    make_get_request "/notes" "search=" 200 "Empty search parameter"
    
    # Test 18: Search with special characters
    make_get_request "/notes" "search=ES6" 200 "Search with numbers/special chars"
    
    # Test 19: Very long search term
    make_get_request "/notes" "search=verylongsearchtermthatprobablywontmatchanything" 200 "Very long search term"
    
    # Test 20: Search for common programming terms
    make_get_request "/notes" "search=function" 200 "Search for programming concepts"
    
    # Test 21: Search for data-related terms
    make_get_request "/notes" "search=data" 200 "Search for data-related content"
    
    # Test 22: Search for web development terms
    make_get_request "/notes" "search=web" 200 "Search for web development terms"
    
    print_status $YELLOW "\n📊 Testing Search Edge Cases..."
    
    # Test 23: Search with URL encoding
    make_get_request "/notes" "search=machine%20learning" 200 "Search with URL encoding"
    
    # Test 24: Search with quotes (expecting 400 since quotes may not be supported)
    make_get_request "/notes" "search=data%20analysis" 200 "Search without quotes (instead of with quotes)"
    
    # Test 25: Search for file extensions/formats
    make_get_request "/notes" "search=CSV" 200 "Search for file formats"
    
    # Test 26: Search combined with limit
    make_get_request "/notes" "search=Python&limit=3" 200 "Search combined with limit"
    
    # Test 27: Search for database terms
    make_get_request "/notes" "search=SQL" 200 "Search for database terms"
    
    # Test 28: Search for optimization terms
    make_get_request "/notes" "search=optimization" 200 "Search for optimization content"
    
    # Test specific note operations
    print_status $YELLOW "\n📋 Testing Note Operations..."
    
    # Test 29: Get specific note (if we have an ID)
    if [ ! -z "$NOTE_ID_1" ]; then
        make_request "GET" "/notes/$NOTE_ID_1" \
            "" \
            200 "Get Specific Note"
        
        # Test 30: Update note
        make_request "PUT" "/notes/$NOTE_ID_1" \
            '{"title":"Advanced JavaScript Tutorial","content":"Updated content covering advanced JavaScript concepts including async/await, promises, and modern ES features."}' \
            200 "Update Note"
        
        # Test 31: Search for updated content
        make_get_request "/notes" "search=async" 200 "Search for updated content"
        
        # Test 32: Search for updated title
        make_get_request "/notes" "search=Advanced" 200 "Search for updated title"
    fi
    
    # Authentication and cleanup tests
    print_status $YELLOW "\n🔐 Testing Authentication Edge Cases..."
    
    # Test 33: Search without authentication (after logout)
    make_request "POST" "/auth/logout" \
        "" \
        200 "User Logout"
    
    # Test 34: Try to search after logout
    make_get_request "/notes" "search=python" 401 "Search without authentication"
    
    # Login again for cleanup
    make_request "POST" "/auth/login" \
        "{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}" \
        200 "Re-login for cleanup"
    
    # Clean up created notes
    print_status $YELLOW "\n🧹 Cleaning up test notes..."
    
    for note_id in "$NOTE_ID_1" "$NOTE_ID_2" "$NOTE_ID_3" "$NOTE_ID_4" "$NOTE_ID_5" "$NOTE_ID_6" "$NOTE_ID_7"; do
        if [ ! -z "$note_id" ]; then
            make_request "DELETE" "/notes/$note_id" \
                "" \
                204 "Delete Note $note_id"
        fi
    done
    
    # Additional authentication tests
    print_status $YELLOW "\n🔒 Testing Authentication Scenarios..."
    
    # Test 35: Try invalid login
    make_request "POST" "/auth/login" \
        '{"email":"invalid@example.com","password":"wrongpassword"}' \
        401 "Invalid Login Attempt"
    
    # Test 36: Try registration with weak password
    make_request "POST" "/auth/register" \
        '{"username":"weakuser","email":"weak@example.com","password":"weak"}' \
        400 "Registration with Weak Password"
    
    # Test 37: Try duplicate registration (expecting 409 Conflict)
    make_request "POST" "/auth/register" \
        "{\"username\":\"$TEST_USERNAME\",\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}" \
        409 "Duplicate User Registration"
    
    # Print summary
    echo -e "\n${YELLOW}📊 Test Summary:${NC}"
    print_status $GREEN "✅ Tests Passed: $TESTS_PASSED"
    print_status $RED "❌ Tests Failed: $TESTS_FAILED"
    
    local total_tests=$((TESTS_PASSED + TESTS_FAILED))
    print_status $BLUE "📈 Total Tests: $total_tests"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        print_status $GREEN "🎉 All tests passed!"
        exit 0
    else
        print_status $RED "💥 Some tests failed!"
        exit 1
    fi
}

# Check if server is running
echo "🔍 Checking if server is running..."
if ! curl -s "$BASE_URL" > /dev/null 2>&1; then
    print_status $RED "❌ Server not reachable at $BASE_URL"
    print_status $YELLOW "💡 Make sure your server is running with: cargo run"
    exit 1
fi

print_status $GREEN "✅ Server is reachable"

# Run main test suite
main