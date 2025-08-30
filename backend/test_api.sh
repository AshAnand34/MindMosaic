#!/bin/bash

# Test script for MindMosaic API
BASE_URL="http://localhost:3000"

echo "🧪 Testing MindMosaic API"
echo "=========================="

# Health check
echo "1. Health check:"
curl -s "$BASE_URL/health" | jq '.'

echo -e "\n2. Register a new user:"
curl -s -X POST "$BASE_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }' | jq '.'

echo -e "\n3. Login:"
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }')

echo "$LOGIN_RESPONSE" | jq '.'

# Extract token and user_id from login response
TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.data.token')
USER_ID=$(echo "$LOGIN_RESPONSE" | jq -r '.data.user_id')

echo -e "\n4. Create a journal entry:"
curl -s -X POST "$BASE_URL/entries" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $USER_ID" \
  -d '{
    "text": "Today was a great day! I felt really productive and happy.",
    "mood_score": 8
  }' | jq '.'

echo -e "\n5. Get all entries:"
curl -s -X GET "$BASE_URL/entries" \
  -H "Authorization: Bearer $USER_ID" | jq '.'

echo -e "\n✅ API tests completed!" 