use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{database::extractor::DatabaseConnection, error::AppError, model::blog::BlogPost};

/// Represents the request body for creating a new blog post.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlogPostBody {
    /// Title of the blog post.
    pub title: String,

    /// Content of the blog post.
    pub content: String,

    /// Category to which the blog post belongs.
    pub category: String,

    /// List of tags associated with the blog post.
    pub tags: Vec<String>,
}

/// Creates a new blog post in the database.
///
/// # Arguments
/// * `DatabaseConnection(mut conn)`: A database connection wrapper.
/// * `Valid(payload)`: A validated JSON payload containing the blog post data.
///
/// # Returns
/// Returns a tuple containing the HTTP status code (`201 Created`) and the newly created blog post in JSON format.
/// If an error occurs, it returns an `AppError`.
///
/// # Errors
/// This function will return an `AppError` if:
/// - The database query fails (e.g., due to connection issues).
/// - The insertion violates a constraint (e.g., unique title or missing fields).
///
/// # Example
/// ```
/// POST /posts
/// {
///     "title": "My First Post",
///     "content": "This is the content of my first post.",
///     "category": "Rust",
///     "tags": ["rust", "async", "sqlx"]
/// }
/// ```
pub async fn create_post(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(payload): Json<BlogPostBody>,
) -> Result<(StatusCode, Json<BlogPost>), AppError> {
    let value = sqlx::query_as!(
        BlogPost,
        r#"
        INSERT INTO blog_posts (title, content, category, tags)
        VALUES ($1, $2, $3, $4)
        RETURNING *;
        "#,
        payload.title,
        payload.content,
        payload.category,
        &payload.tags
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok((StatusCode::CREATED, Json(value)))
}
