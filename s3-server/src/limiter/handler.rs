use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn is_req_limited(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    Ok(next.run(req).await)
}
