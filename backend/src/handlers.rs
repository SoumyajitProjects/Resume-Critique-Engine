use warp::{multipart::FormData, Rejection, Reply};
use warp::http::StatusCode;
use std::sync::Arc;
use sqlx::PgPool;
use futures_util::TryStreamExt;
use bytes::BufMut;
use std::fs;
use std::io::Write;
use uuid::Uuid;

use crate::config::Config;
use crate::models::*;
use crate::auth::{self, Claims, AuthError};

pub async fn upload_resume(
    form: FormData,
    db_pool: Arc<PgPool>,
    config: Arc<Config>,
) -> Result<impl Reply, Rejection> {
    // Extract user from auth header - for now we'll use a default user
    let user_id = 1; // TODO: Get from JWT token
    
    let parts: Vec<_> = form.try_collect().await.map_err(|_| warp::reject())?;
    
    let mut file_data = Vec::new();
    let mut filename = String::new();
    let mut content_type = String::new();
    
    for part in parts {
        if part.name() == "resume" {
            filename = part.filename().unwrap_or("resume.pdf").to_string();
            content_type = part.content_type().unwrap_or("application/pdf").to_string();
            
            let stream = part.stream();
            let data: Vec<u8> = stream
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|_| warp::reject())?;
            
            file_data = data;
            break;
        }
    }
    
    if file_data.is_empty() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "No file uploaded"})),
            StatusCode::BAD_REQUEST,
        ));
    }
    
    // Create uploads directory if it doesn't exist
    fs::create_dir_all(&config.upload_dir).map_err(|_| warp::reject())?;
    
    // Save file to disk
    let file_id = Uuid::new_v4().to_string();
    let file_path = format!("{}/{}-{}", config.upload_dir, file_id, filename);
    let mut file = fs::File::create(&file_path).map_err(|_| warp::reject())?;
    file.write_all(&file_data).map_err(|_| warp::reject())?;
    
    // Extract text content (simplified - in production you'd use proper PDF parsing)
    let content = match content_type.as_str() {
        "application/pdf" => extract_pdf_text(&file_path).await.unwrap_or_else(|_| "PDF content extraction failed".to_string()),
        "text/plain" => String::from_utf8_lossy(&file_data).to_string(),
        _ => "Unsupported file type".to_string(),
    };
    
    // Save resume to database
    let resume = sqlx::query_as!(
        Resume,
        r#"
        INSERT INTO resumes (user_id, filename, original_content, file_path, file_size, mime_type)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, filename, original_content, file_path, file_size, mime_type, uploaded_at
        "#,
        user_id,
        filename,
        content,
        Some(file_path),
        Some(file_data.len() as i32),
        Some(content_type)
    )
    .fetch_one(&**db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        warp::reject()
    })?;
    
    // Call AI service for critique
    let ai_request = AiCritiqueRequest {
        resume_text: content,
        filename: filename.clone(),
    };
    
    let client = reqwest::Client::new();
    let ai_response = client
        .post(&format!("{}/critique", config.ai_service_url))
        .json(&ai_request)
        .send()
        .await
        .map_err(|e| {
            eprintln!("AI service error: {}", e);
            warp::reject()
        })?;
    
    if !ai_response.status().is_success() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "AI service failed"})),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
    
    let ai_critique: AiCritiqueResponse = ai_response
        .json()
        .await
        .map_err(|_| warp::reject())?;
    
    // Save critique to database
    let critique = sqlx::query_as!(
        Critique,
        r#"
        INSERT INTO critiques (
            resume_id, overall_score, structure_score, keywords_score, 
            action_verbs_score, quantified_impact_score, readability_score,
            detailed_feedback, improvement_suggestions
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, resume_id, overall_score, structure_score, keywords_score,
                  action_verbs_score, quantified_impact_score, readability_score,
                  detailed_feedback, improvement_suggestions, created_at
        "#,
        resume.id,
        ai_critique.overall_score,
        ai_critique.structure_score,
        ai_critique.keywords_score,
        ai_critique.action_verbs_score,
        ai_critique.quantified_impact_score,
        ai_critique.readability_score,
        ai_critique.detailed_feedback,
        ai_critique.improvement_suggestions
    )
    .fetch_one(&**db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        warp::reject()
    })?;
    
    let response = UploadResponse {
        message: "Resume uploaded and analyzed successfully".to_string(),
        critique_id: critique.id,
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

pub async fn get_critique(
    critique_id: i32,
    db_pool: Arc<PgPool>,
    _config: Arc<Config>,
) -> Result<impl Reply, Rejection> {
    let critique = sqlx::query!(
        r#"
        SELECT c.*, r.filename 
        FROM critiques c
        JOIN resumes r ON c.resume_id = r.id
        WHERE c.id = $1
        "#,
        critique_id
    )
    .fetch_optional(&**db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        warp::reject()
    })?;
    
    match critique {
        Some(c) => {
            let response = CritiqueResponse {
                id: c.id,
                resume_filename: c.filename,
                overall_score: c.overall_score,
                scores: CritiqueScores {
                    structure: c.structure_score,
                    keywords: c.keywords_score,
                    action_verbs: c.action_verbs_score,
                    quantified_impact: c.quantified_impact_score,
                    readability: c.readability_score,
                },
                detailed_feedback: c.detailed_feedback,
                improvement_suggestions: c.improvement_suggestions,
                created_at: c.created_at,
            };
            
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        None => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Critique not found"})),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn login(
    request: LoginRequest,
    db_pool: Arc<PgPool>,
    config: Arc<Config>,
) -> Result<impl Reply, Rejection> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        request.email
    )
    .fetch_optional(&**db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        warp::reject()
    })?;
    
    match user {
        Some(u) => {
            if auth::verify_password(&request.password, &u.password_hash).unwrap_or(false) {
                let token = auth::create_jwt(u.id, &u.email, &config.jwt_secret)
                    .map_err(|_| warp::reject())?;
                
                let response = AuthResponse {
                    token,
                    user: UserResponse {
                        id: u.id,
                        email: u.email,
                        name: u.name,
                    },
                };
                
                Ok(warp::reply::with_status(
                    warp::reply::json(&response),
                    StatusCode::OK,
                ))
            } else {
                Ok(warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({"error": "Invalid credentials"})),
                    StatusCode::UNAUTHORIZED,
                ))
            }
        }
        None => Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "User not found"})),
            StatusCode::NOT_FOUND,
        )),
    }
}

pub async fn register(
    request: RegisterRequest,
    db_pool: Arc<PgPool>,
    config: Arc<Config>,
) -> Result<impl Reply, Rejection> {
    let password_hash = auth::hash_password(&request.password)
        .map_err(|_| warp::reject())?;
    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, $2, $3)
        RETURNING id, email, password_hash, name, created_at, updated_at
        "#,
        request.email,
        password_hash,
        request.name
    )
    .fetch_one(&**db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        warp::reject()
    })?;
    
    let token = auth::create_jwt(user.id, &user.email, &config.jwt_secret)
        .map_err(|_| warp::reject())?;
    
    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        },
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::CREATED,
    ))
}

pub async fn get_current_user(claims: Claims) -> Result<impl Reply, Rejection> {
    let user_response = UserResponse {
        id: claims.sub,
        email: claims.email,
        name: "Current User".to_string(), // In real app, fetch from DB
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&user_response),
        StatusCode::OK,
    ))
}

pub async fn get_history(
    claims: Claims,
    db_pool: Arc<PgPool>,
) -> Result<impl Reply, Rejection> {
    let critiques = sqlx::query!(
        r#"
        SELECT c.*, r.filename 
        FROM critiques c
        JOIN resumes r ON c.resume_id = r.id
        WHERE r.user_id = $1
        ORDER BY c.created_at DESC
        "#,
        claims.sub
    )
    .fetch_all(&**db_pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        warp::reject()
    })?;
    
    let critique_responses: Vec<CritiqueResponse> = critiques
        .into_iter()
        .map(|c| CritiqueResponse {
            id: c.id,
            resume_filename: c.filename,
            overall_score: c.overall_score,
            scores: CritiqueScores {
                structure: c.structure_score,
                keywords: c.keywords_score,
                action_verbs: c.action_verbs_score,
                quantified_impact: c.quantified_impact_score,
                readability: c.readability_score,
            },
            detailed_feedback: c.detailed_feedback,
            improvement_suggestions: c.improvement_suggestions,
            created_at: c.created_at,
        })
        .collect();
    
    let response = HistoryResponse {
        critiques: critique_responses,
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if err.find::<AuthError>().is_some() {
        code = StatusCode::UNAUTHORIZED;
        message = "Unauthorized";
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("Unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&serde_json::json!({
        "error": message
    }));

    Ok(warp::reply::with_status(json, code))
}

// Helper function for PDF text extraction (simplified)
async fn extract_pdf_text(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // In a real implementation, you'd use a proper PDF library
    // For now, return a placeholder
    Ok(format!("PDF content from file: {}", file_path))
}
