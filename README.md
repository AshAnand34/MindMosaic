![MindMosaic Logo](transparentlogo.png)

## MindMosaic – Mental Health Companion App

### 📌 Overview

MindMosaic is a web-based mental health companion application. It helps users track emotional well-being via journaling and mood logs. It uses natural language processing (NLP) to detect sentiment and emotion from journal entries and provides personalized insights, all while ensuring privacy and data security.

---

### 🏗️ Architecture

```plaintext
[Angular Frontend]
        |
        ↓
[Rust Backend API – Axum]
        |
        | REST API
        ↓
[Python ML Microservice – FastAPI]
        |
        ↓
[MongoDB Database]
```

---

### 🔧 Tech Stack

| Layer         | Technology                                   |
| ------------- | -------------------------------------------- |
| Frontend      | Angular 17                                   |
| Backend API   | Rust + Axum                                  |
| ML Service    | Python + FastAPI + Transformers              |
| Database      | MongoDB                                      |
| Deployment    | Docker Compose                               |
| Communication | REST over HTTP, JSON payloads                |
| ML Libraries  | Transformers (Hugging Face), TextBlob, VADER |

---

### 🔐 Authentication and Authorization

* JWT-based Auth handled in Rust backend
* Frontend stores token securely and passes in Authorization header
* No direct auth to ML microservice or DB

---

### 🧠 Features & Functional Requirements

#### ✅ MVP Features

1. User Registration and Login (JWT auth)
2. Journal Entry Submission
3. Sentiment and Emotion Detection
4. Mood Tracker (emoji or scale)
5. Insight Dashboard with trends and suggestions

---

### 🗃️ Data Models

#### MongoDB Collections

**users**

```json
{
  "_id": "ObjectId",
  "email": "string",
  "hashed_password": "string",
  "created_at": "datetime"
}
```

**journal\_entries**

```json
{
  "_id": "ObjectId",
  "user_id": "ObjectId",
  "text": "string",
  "timestamp": "datetime",
  "sentiment": "string",
  "emotions": ["string"],
  "mood_score": 3
}
```

---

### 📡 API Design

**Rust API Endpoints**

| Method | Endpoint         | Description                |
| ------ | ---------------- | -------------------------- |
| POST   | `/auth/register` | Create user account        |
| POST   | `/auth/login`    | Login & return JWT         |
| GET    | `/entries`       | Get user's journal entries |
| POST   | `/entries`       | Submit new journal entry   |
| GET    | `/insights`      | Get emotion/mood trends    |

**Python ML API**

| Method | Endpoint   | Description                                                    |
| ------ | ---------- | -------------------------------------------------------------- |
| POST   | `/analyze` | Accepts journal entry and returns sentiment + emotion analysis |

---

### 🧪 ML Pipeline (Python)

1. Preprocessing: Tokenization, normalization
2. Sentiment Analysis: VADER or BERT
3. Emotion Detection: HuggingFace classifier
4. Return Result:

```json
{
  "sentiment": "negative",
  "emotions": ["anxiety", "fear"]
}
```

---

### ⚙️ DevOps and Deployment

**Docker Compose Services:**

* rust-api
* ml-api
* mongo

---

### 📈 Future Enhancements

* Voice Journaling (Whisper)
* Smart Reminders
* Mindfulness Recommendations
* Offline ML inference (ONNX in Rust)
* Mobile App

---

### 🧪 Testing Strategy

| Component | Tool                    | Focus                          |
| --------- | ----------------------- | ------------------------------ |
| Rust API  | tokio::test, Postman    | Routing, DB ops, auth          |
| Python ML | pytest, requests        | Sentiment and emotion accuracy |
| Frontend  | Cypress or Jasmine      | UX flow, API integration       |
| E2E       | Docker Compose test env | Full workflow validation       |

---

### 📁 Folder Structure (Simplified)

```
/frontend
/backend
  └── src/
  └── Cargo.toml
/ml
  └── app/
  └── requirements.txt
/docker-compose.yml
```
