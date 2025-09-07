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
    
    # Test 4: Create a note
    local note_response=$(curl -s -X POST "$BASE_URL/notes" \
        -H "Content-Type: application/json" \
        -b $COOKIES_FILE \
        -c $COOKIES_FILE \
        -d '{"title":"Test Note","content":"This is a test note.","tags":["test"]}' \
        -w "HTTPSTATUS:%{http_code}")
    
    local note_http_code=$(echo $note_response | tr -d '\n' | sed -e 's/.*HTTPSTATUS://')
    local note_body=$(echo $note_response | sed -e 's/HTTPSTATUS:.*//g')
    
    echo -e "\n${BLUE}Testing: Create Note${NC}"
    echo "Response: $note_body"
    
    if [ "$note_http_code" -eq "201" ]; then
        print_status $GREEN "✅ Status: $note_http_code (Expected: 201)"
        ((TESTS_PASSED++))
        # Extract note ID if present for future tests
        NOTE_ID=$(echo $note_body | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
    else
        print_status $RED "❌ Status: $note_http_code (Expected: 201)"
        ((TESTS_FAILED++))
    fi
    
    # Test 5: Get all notes
    make_request "GET" "/notes" \
        "" \
        200 "Get All Notes"
    
    # Test 6: Get specific note (if we have an ID)
    if [ ! -z "$NOTE_ID" ]; then
        make_request "GET" "/notes/$NOTE_ID" \
            "" \
            200 "Get Specific Note"
        
        # Test 7: Update note
        make_request "PUT" "/notes/$NOTE_ID" \
            '{"title":"Updated Test Note","content":"This note has been updated.","tags":["test","updated"]}' \
            200 "Update Note"
        
        # Test 8: Delete note (expecting 204 No Content)
        make_request "DELETE" "/notes/$NOTE_ID" \
            "" \
            204 "Delete Note"
    fi
    
    # Test 9: Try to access protected route without auth (after logout)
    make_request "POST" "/auth/logout" \
        "" \
        200 "User Logout"
    
    # Test 10: Try to access protected route after logout
    make_request "GET" "/auth/me" \
        "" \
        401 "Access Protected Route After Logout"
    
    # Test 11: Try invalid login
    make_request "POST" "/auth/login" \
        '{"email":"invalid@example.com","password":"wrongpassword"}' \
        401 "Invalid Login Attempt"
    
    # Test 12: Try registration with weak password
    make_request "POST" "/auth/register" \
        '{"username":"weakuser","email":"weak@example.com","password":"weak"}' \
        400 "Registration with Weak Password"
    
    # Test 13: Try duplicate registration (expecting 409 Conflict)
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