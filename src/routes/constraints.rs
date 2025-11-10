use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::{api_error::ApiError, app_state::AppState, domain::*};

#[derive(Serialize, Deserialize)]
pub struct CreateConstraintRequest {
    pub constraint: Constraint,
}

#[derive(Serialize, Deserialize)]
pub struct ConstraintsListResponse {
    pub constraints: Vec<Constraint>,
}

pub async fn create_constraint(
    State(state): State<AppState>,
    Json(request): Json<CreateConstraintRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut constraints = state.constraints.write().await;
    constraints.push(request.constraint);
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "message": "Constraint created" }))))
}

pub async fn list_constraints(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let constraints = state.constraints.read().await;
    Ok(Json(ConstraintsListResponse { constraints: constraints.clone() }))
}

pub async fn update_constraint(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(request): Json<CreateConstraintRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut constraints = state.constraints.write().await;

    if let Some(existing) = constraints.iter_mut().find(|c| c.name == name) {
        *existing = request.constraint;
        Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Constraint updated" }))))
    } else {
        Err(ApiError::ItemNotFound)
    }
}

pub async fn delete_constraint(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut constraints = state.constraints.write().await;

    let before = constraints.len();
    constraints.retain(|c| c.name != name);

    if constraints.len() < before {
        Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Constraint deleted" }))))
    } else {
        Err(ApiError::ItemNotFound)
    }
}
