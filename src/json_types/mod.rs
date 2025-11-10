use crate::domain::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub item_set_type: SetType,
    pub schema: Schema,
}

#[derive(Serialize, Deserialize)]
pub struct AddMemberRequest {
    pub item_name: String,
    pub member: Member,
}

#[derive(Serialize, Deserialize)]
pub struct CreateConstraintRequest {
    pub constraint: Constraint,
}

#[derive(Serialize, Deserialize)]
pub struct GetItemsResponse {
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetMembersResponse {
    pub item_name: String,
    pub members: Vec<MemberSummary>,
}

#[derive(Serialize, Deserialize)]
pub struct MemberSummary {
    pub id: ItemId,
    pub display_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetFieldsResponse {
    pub item_name: String,
    pub fields: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProblemDataFile {
    pub items: Vec<ItemData>,
    pub constraints: Vec<Constraint>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub item_set_type: SetType,
    pub schema: Schema,
    pub members: Vec<Member>,
}
