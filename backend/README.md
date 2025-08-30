# MindMosaic Backend API

A Rust-based REST API for the MindMosaic mental health companion application, built with Axum and MongoDB.

## 🚀 Features

- **User Authentication**: JWT-based registration and login
- **Journal Entries**: Create and retrieve personal journal entries
- **Mood Tracking**: Optional mood scoring with entries
- **MongoDB Integration**: Async MongoDB operations with proper error handling
- **CORS Support**: Cross-origin resource sharing enabled
- **RESTful API**: Clean, consistent API design

## 📋 Prerequisites

- Rust (latest stable version)
- MongoDB (running locally or accessible via connection string)
- `jq` (for testing - optional)

## 🛠️ Setup

1. **Clone and navigate to the backend directory:**
   ```bash
   cd backend
   ```

2. **Copy environment variables:**
   ```bash
   cp env.example .env
   ```

3. **Edit `.env` with your configuration:**
   ```env
   MONGODB_URI=mongodb://localhost:27017
   ```

4. **Install dependencies and run:**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:3000`

## 📚 API Endpoints

### Authentication

#### `POST /auth/register`
Register a new user account.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "token": "uuid-token",
    "user_id": "507f1f77bcf86cd799439011"
  },
  "message": null
}
```

#### `POST /auth/login`
Login with existing credentials.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:** Same as register endpoint.

### Journal Entries

#### `POST /entries`
Create a new journal entry.

**Headers:**
```
Authorization: Bearer <user_id>
Content-Type: application/json
```

**Request Body:**
```json
{
  "text": "Today was a great day! I felt really productive.",
  "mood_score": 8
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "_id": "507f1f77bcf86cd799439012",
    "user_id": "507f1f77bcf86cd799439011",
    "text": "Today was a great day! I felt really productive.",
    "timestamp": "2024-01-15T10:30:00Z",
    "sentiment": null,
    "emotions": null,
    "mood_score": 8
  },
  "message": null
}
```

#### `GET /entries`
Retrieve all journal entries for the authenticated user.

**Headers:**
```
Authorization: Bearer <user_id>
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "_id": "507f1f77bcf86cd799439012",
      "user_id": "507f1f77bcf86cd799439011",
      "text": "Today was a great day!",
      "timestamp": "2024-01-15T10:30:00Z",
      "sentiment": null,
      "emotions": null,
      "mood_score": 8
    }
  ],
  "message": null
}
```

### Health Check

#### `GET /health`
Check API status.

**Response:**
```json
{
  "success": true,
  "data": "MindMosaic API is running!",
  "message": null
}
```

## 🧪 Testing

Run the test script to verify all endpoints:

```bash
chmod +x test_api.sh
./test_api.sh
```

**Note:** Make sure you have `jq` installed for JSON formatting in the test output.

## 🗄️ Database Schema

### Users Collection
```json
{
  "_id": "ObjectId",
  "email": "string",
  "hashed_password": "string",
  "created_at": "datetime"
}
```

### Journal Entries Collection
```json
{
  "_id": "ObjectId",
  "user_id": "ObjectId",
  "text": "string",
  "timestamp": "datetime",
  "sentiment": "string (optional)",
  "emotions": ["string"] (optional),
  "mood_score": "integer (optional)"
}
```

## 🔧 Development

### Project Structure
```
backend/
├── src/
│   └── main.rs          # Main application entry point
├── Cargo.toml           # Rust dependencies
├── .env                 # Environment variables (create from env.example)
├── env.example          # Example environment configuration
├── test_api.sh          # API testing script
└── README.md            # This file
```

### Key Dependencies
- **axum**: Web framework
- **mongodb**: MongoDB driver
- **serde**: Serialization/deserialization
- **chrono**: Date/time handling
- **bcrypt**: Password hashing
- **jsonwebtoken**: JWT authentication
- **tower-http**: CORS middleware

## 🚨 Security Notes

1. **JWT Implementation**: The current implementation uses UUIDs as tokens. In production, implement proper JWT with expiration and refresh tokens.
2. **Password Hashing**: Passwords are hashed using bcrypt with default cost.
3. **CORS**: Currently allows all origins. Configure appropriately for production.
4. **Environment Variables**: Never commit `.env` files to version control.

## 🔄 Next Steps

- [ ] Implement proper JWT authentication
- [ ] Add input validation and sanitization
- [ ] Implement rate limiting
- [ ] Add comprehensive error handling
- [ ] Create database migrations
- [ ] Add unit and integration tests
- [ ] Implement sentiment analysis integration
- [ ] Add pagination for entries
- [ ] Implement user profile management

## 📝 License

This project is part of MindMosaic and follows the same license terms. 