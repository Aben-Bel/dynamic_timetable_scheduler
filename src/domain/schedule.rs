use super::item::ItemId;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Assignment {
    pub task_id: ItemId,
    pub task_item_name: String,
    pub resources: HashMap<String, ItemId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub assignments: Vec<Assignment>,
}

impl Schedule {
    pub fn new(assignments: Vec<Assignment>) -> Self {
        Self { assignments }
    }

    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
}
