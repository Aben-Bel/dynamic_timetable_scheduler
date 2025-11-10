use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ItemId(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SetType {
    B_Set,
    E_Set,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(i32),
    Date(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Integer,
    DateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldSchema {
    pub field_name: String,
    pub field_type: FieldType,
    pub is_required: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub definitions: HashMap<String, FieldSchema>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: ItemId,
    pub fields: HashMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub item_set_type: SetType,
    pub members: Vec<Member>,
    pub schema: Schema,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProblemData {
    pub item_categories: HashMap<String, Item>,
}
