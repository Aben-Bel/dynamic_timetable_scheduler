use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::{api_error::ApiError, app_state::AppState, domain::*};

#[derive(Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub item_set_type: SetType,
    pub schema: Schema,
}

#[derive(Serialize, Deserialize)]
pub struct ItemResponse {
    pub name: String,
    pub item_set_type: SetType,
    pub member_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ItemsListResponse {
    pub items: Vec<ItemResponse>,
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(request): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut problem_data = state.problem_data.write().await;
    
    let item = Item {
        name: request.name.clone(),
        item_set_type: request.item_set_type,
        members: vec![],
        schema: request.schema,
    };
    
    problem_data.item_categories.insert(request.name, item);
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "message": "Item created" }))))
}

pub async fn list_items(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let problem_data = state.problem_data.read().await;
    
    let items: Vec<ItemResponse> = problem_data.item_categories.values()
        .map(|item| ItemResponse {
            name: item.name.clone(),
            item_set_type: item.item_set_type,
            member_count: item.members.len(),
        })
        .collect();
    
    Ok(Json(ItemsListResponse { items }))
}

#[derive(Serialize, Deserialize)]
pub struct UpdateItemRequest {
    pub item_set_type: SetType,
    pub schema: Schema,
}

pub async fn update_item(
    State(state): State<AppState>,
    Path(item_name): Path<String>,
    Json(request): Json<UpdateItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut problem_data = state.problem_data.write().await;

    let item = problem_data.item_categories.get_mut(&item_name)
        .ok_or(ApiError::ItemNotFound)?;

    item.item_set_type = request.item_set_type;
    item.schema = request.schema;

    Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Item updated" }))))
}

pub async fn delete_item(
    State(state): State<AppState>,
    Path(item_name): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let mut problem_data = state.problem_data.write().await;

    if problem_data.item_categories.remove(&item_name).is_some() {
        Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Item deleted" }))))
    } else {
        Err(ApiError::ItemNotFound)
    }
}
