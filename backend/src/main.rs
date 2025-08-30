use axum::{
    extract::{State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection, Database,
};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener};
use tower_http::cors::{Any, CorsLayer};
use futures_util::TryStreamExt;
use uuid::Uuid;

// Data Models
#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    email: String,
    hashed_password: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JournalEntry {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    user_id: ObjectId,
    text: String,
    timestamp: DateTime<Utc>,
    sentiment: Option<String>,
    emotions: Option<Vec<String>>,
    mood_score: Option<i32>,
}

// Request/Response Models
#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct CreateEntryRequest {
    text: String,
    mood_score: Option<i32>,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
    user_id: String,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

// App State
#[derive(Clone)]
struct AppState {
    db: Database,
}

// Database Operations
impl AppState {
    async fn create_user(&self, email: String, hashed_password: String) -> Result<User, mongodb::error::Error> {
        let collection: Collection<User> = self.db.collection("users");
        
        let user = User {
            id: None,
            email,
            hashed_password,
            created_at: Utc::now(),
        };
        
        let result = collection.insert_one(&user, None).await?;
        let mut user = user;
        user.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(user)
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, mongodb::error::Error> {
        let collection: Collection<User> = self.db.collection("users");
        let filter = doc! { "email": email };
        collection.find_one(filter, None).await
    }

    async fn create_journal_entry(&self, user_id: ObjectId, text: String, mood_score: Option<i32>) -> Result<JournalEntry, mongodb::error::Error> {
        let collection: Collection<JournalEntry> = self.db.collection("journal_entries");
        
        let entry = JournalEntry {
            id: None,
            user_id,
            text,
            timestamp: Utc::now(),
            sentiment: None,
            emotions: None,
            mood_score,
        };
        
        let result = collection.insert_one(&entry, None).await?;
        let mut entry = entry;
        entry.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(entry)
    }

    async fn get_user_entries(&self, user_id: ObjectId) -> Result<Vec<JournalEntry>, mongodb::error::Error> {
        let collection: Collection<JournalEntry> = self.db.collection("journal_entries");
        let filter = doc! { "user_id": user_id };
        let mut cursor = collection.find(filter, None).await?;
         
        let mut entries = Vec::new();
        while let Some(entry) = cursor.try_next().await? {
            entries.push(entry);
        }
        Ok(entries)
    }
}

// Route Handlers
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    // Check if user already exists
    if let Some(_) = state.find_user_by_email(&payload.email).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("User already exists".to_string()),
        }));
    }

    // Hash password
    let hashed_password = bcrypt::hash(payload.password.as_bytes(), bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create user
    let user = state.create_user(payload.email, hashed_password).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Generate JWT token (simplified - you'd want proper JWT handling)
    let token = Uuid::new_v4().to_string();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(AuthResponse {
            token,
            user_id: user.id.unwrap().to_hex(),
        }),
        message: None,
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    let user = state.find_user_by_email(&payload.email).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let is_valid = bcrypt::verify(payload.password.as_bytes(), &user.hashed_password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Invalid credentials".to_string()),
        }));
    }

    // Generate JWT token
    let token = Uuid::new_v4().to_string();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(AuthResponse {
            token,
            user_id: user.id.unwrap().to_hex(),
        }),
        message: None,
    }))
}

async fn create_entry(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateEntryRequest>,
) -> Result<Json<ApiResponse<JournalEntry>>, StatusCode> {
    // Extract user_id from Authorization header (simplified)
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // In a real app, you'd decode the JWT token here
    // For now, we'll assume the header contains the user_id directly
    let user_id = auth_header.strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let object_id = ObjectId::parse_str(user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let entry = state.create_journal_entry(object_id, payload.text, payload.mood_score).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(entry),
        message: None,
    }))
}

async fn get_entries(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<Vec<JournalEntry>>>, StatusCode> {
    // Extract user_id from Authorization header (simplified)
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id = auth_header.strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let object_id = ObjectId::parse_str(user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let entries = state.get_user_entries(object_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(entries),
        message: None,
    }))
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("MindMosaic API is running!".to_string()),
        message: None,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // MongoDB connection
    let mongo_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database("mindmosaic");
    
    // Test the connection
    db.run_command(doc! {"ping": 1}, None).await?;
    println!("✅ Connected to MongoDB");

    let state = AppState { db };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/entries", get(get_entries))
        .route("/entries", post(create_entry))
        .layer(cors)
        .with_state(state);

    // Run it
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 MindMosaic API listening on http://127.0.0.1:3000");
    println!("📊 Health check: http://127.0.0.1:3000/health");
    
    axum::serve(listener, app).await?;
    Ok(())
}