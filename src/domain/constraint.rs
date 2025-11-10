use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintMode {
    Forbid,
    Require,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    In,
    NotIn,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Before,
    After,
    Overlap,
    NoOverlap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    pub item_name: String,
    pub field_key: String,
    pub operator: ComparisonOperator,
    pub target_values: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConstraintRule {
    MultiAssignmentCheck {
        conditions: Vec<Condition>,
        logical_op: LogicalOperator,
        mode: ConstraintMode,
    },
    GlobalAllDifferent {
        unique_item_field: String,
        group_item_field: String,
    },
    GlobalCardinality {
        target_item_field: String,
        max_count: u32,
        scope_conditions: Option<Vec<Condition>>,
    },
    GlobalTemporalPrecedence {
        grouping_item_field: String,
        first_conditions: Vec<Condition>,
        second_conditions: Vec<Condition>,
        temporal_relation: ComparisonOperator,
        temporal_fields: Vec<String>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub weight: u32,
    pub rule: ConstraintRule,
}
