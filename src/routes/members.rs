use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{api_error::ApiError, app_state::AppState, domain::*};

#[derive(Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub id: ItemId,
    pub fields: std::collections::HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberResponse {
    pub id: ItemId,
    pub fields: std::collections::HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct MembersListResponse {
    pub members: Vec<MemberResponse>,
}

pub async fn add_member(
    State(state): State<AppState>,
    Path(item_name): Path<String>,
    Json(request): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut problem_data = state.problem_data.write().await;
    
    let item = problem_data.item_categories.get_mut(&item_name)
        .ok_or(ApiError::ItemNotFound)?;
    
    let member = Member {
        id: request.id,
        fields: request.fields,
    };
    
    item.members.push(member);
    
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "message": "Member added" }))))
}

pub async fn list_members(
    State(state): State<AppState>,
    Path(item_name): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let problem_data = state.problem_data.read().await;
    
    let item = problem_data.item_categories.get(&item_name)
        .ok_or(ApiError::ItemNotFound)?;
    
    let members: Vec<MemberResponse> = item.members.iter()
        .map(|m| MemberResponse {
            id: m.id,
            fields: m.fields.clone(),
        })
        .collect();
    
    Ok(Json(MembersListResponse { members }))
}

pub async fn update_member(
    State(state): State<AppState>,
    Path((item_name, member_id)): Path<(String, u32)>,
    Json(request): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let mut problem_data = state.problem_data.write().await;

    let item = problem_data.item_categories.get_mut(&item_name)
        .ok_or(ApiError::ItemNotFound)?;

    if let Some(member) = item.members.iter_mut().find(|m| m.id.0 == member_id) {
        member.fields = request.fields;
        Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Member updated" }))))
    } else {
        Err(ApiError::MemberNotFound)
    }
}

pub async fn delete_member(
    State(state): State<AppState>,
    Path((item_name, member_id)): Path<(String, u32)>,
) -> Result<impl IntoResponse, ApiError> {
    let mut problem_data = state.problem_data.write().await;

    let item = problem_data.item_categories.get_mut(&item_name)
        .ok_or(ApiError::ItemNotFound)?;

    let before = item.members.len();
    item.members.retain(|m| m.id.0 != member_id);

    if item.members.len() < before {
        Ok((StatusCode::OK, Json(serde_json::json!({ "message": "Member deleted" }))))
    } else {
        Err(ApiError::MemberNotFound)
    }
}
