use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::*;

pub type ProblemDataType = Arc<RwLock<ProblemData>>;
pub type ConstraintsType = Arc<RwLock<Vec<Constraint>>>;

#[derive(Clone)]
pub struct AppState {
    pub problem_data: ProblemDataType,
    pub constraints: ConstraintsType,
}

impl AppState {
    pub fn new(problem_data: ProblemDataType, constraints: ConstraintsType) -> Self {
        Self { problem_data, constraints }
    }
}
