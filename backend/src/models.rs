use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Resume {
    pub id: i32,
    pub user_id: i32,
    pub filename: String,
    pub original_content: String,
    pub file_path: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Critique {
    pub id: i32,
    pub resume_id: i32,
    pub overall_score: f32,
    pub structure_score: f32,
    pub keywords_score: f32,
    pub action_verbs_score: f32,
    pub quantified_impact_score: f32,
    pub readability_score: f32,
    pub detailed_feedback: serde_json::Value,
    pub improvement_suggestions: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FeedbackHistory {
    pub id: i32,
    pub user_id: i32,
    pub resume_id: i32,
    pub critique_id: i32,
    pub version_number: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CritiqueResponse {
    pub id: i32,
    pub resume_filename: String,
    pub overall_score: f32,
    pub scores: CritiqueScores,
    pub detailed_feedback: serde_json::Value,
    pub improvement_suggestions: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CritiqueScores {
    pub structure: f32,
    pub keywords: f32,
    pub action_verbs: f32,
    pub quantified_impact: f32,
    pub readability: f32,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub message: String,
    pub critique_id: i32,
}

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub critiques: Vec<CritiqueResponse>,
}

// AI Service Request/Response
#[derive(Debug, Serialize)]
pub struct AiCritiqueRequest {
    pub resume_text: String,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
pub struct AiCritiqueResponse {
    pub overall_score: f32,
    pub structure_score: f32,
    pub keywords_score: f32,
    pub action_verbs_score: f32,
    pub quantified_impact_score: f32,
    pub readability_score: f32,
    pub detailed_feedback: serde_json::Value,
    pub improvement_suggestions: serde_json::Value,
}
