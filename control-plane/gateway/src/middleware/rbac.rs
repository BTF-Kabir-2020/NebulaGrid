use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};

use crate::state::Claims;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Role {
    Admin,
    Operator,
    Viewer,
}

#[allow(dead_code)]
pub async fn require_role(role: Role, req: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = req.extensions().get::<Claims>().ok_or(StatusCode::UNAUTHORIZED)?;

    let has_role = match role {
        Role::Admin => claims.roles.iter().any(|r| r == "admin"),
        Role::Operator => claims.roles.iter().any(|r| r == "admin" || r == "operator"),
        Role::Viewer => true,
    };

    if !has_role {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}
