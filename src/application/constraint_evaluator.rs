use crate::domain::*;
use std::collections::{HashMap, HashSet};

pub fn evaluate_schedule(
    schedule: &Schedule,
    problem_data: &ProblemData,
    constraints: &[Constraint],
) -> u32 {
    let mut total_cost = 0;
    
    for constraint in constraints {
        let violations = evaluate_constraint(&constraint.rule, schedule, problem_data);
        total_cost += violations * constraint.weight;
    }
    
    total_cost
}

fn evaluate_constraint(
    rule: &ConstraintRule,
    schedule: &Schedule,
    problem_data: &ProblemData,
) -> u32 {
    match rule {
        ConstraintRule::MultiAssignmentCheck { conditions, logical_op, mode } => {
            evaluate_multi_assignment(schedule, problem_data, conditions, logical_op, mode)
        }
        ConstraintRule::GlobalAllDifferent { unique_item_field, group_item_field } => {
            evaluate_all_different(schedule, problem_data, unique_item_field, group_item_field)
        }
        ConstraintRule::GlobalCardinality { target_item_field, max_count, scope_conditions } => {
            evaluate_cardinality(schedule, problem_data, target_item_field, *max_count, scope_conditions)
        }
        ConstraintRule::GlobalTemporalPrecedence { 
            grouping_item_field, 
            first_conditions, 
            second_conditions, 
            temporal_relation, 
            temporal_fields 
        } => {
            evaluate_temporal_precedence(
                schedule, 
                problem_data, 
                grouping_item_field, 
                first_conditions, 
                second_conditions, 
                temporal_relation, 
                temporal_fields
            )
        }
    }
}

fn evaluate_multi_assignment(
    schedule: &Schedule,
    problem_data: &ProblemData,
    conditions: &[Condition],
    logical_op: &LogicalOperator,
    mode: &ConstraintMode,
) -> u32 {
    let mut violations = 0;
    
    for assignment in &schedule.assignments {
        let results: Vec<bool> = conditions.iter()
            .map(|c| evaluate_condition(c, assignment, problem_data))
            .collect();
        
        let combined = match logical_op {
            LogicalOperator::And => results.iter().all(|&b| b),
            LogicalOperator::Or => results.iter().any(|&b| b),
        };
        
        match mode {
            ConstraintMode::Forbid if combined => violations += 1,
            ConstraintMode::Require if !combined => violations += 1,
            _ => {}
        }
    }
    
    violations
}

fn evaluate_condition(
    condition: &Condition,
    assignment: &Assignment,
    problem_data: &ProblemData,
) -> bool {
    let member_id = if condition.item_name == assignment.task_item_name {
        assignment.task_id
    } else {
        match assignment.resources.get(&condition.item_name) {
            Some(&id) => id,
            None => return false,
        }
    };
    
    let item = match problem_data.item_categories.get(&condition.item_name) {
        Some(i) => i,
        None => return false,
    };
    
    let member = match item.members.iter().find(|m| m.id == member_id) {
        Some(m) => m,
        None => return false,
    };
    
    let value = if condition.field_key == "id" {
        Value::Number(member.id.0 as i32)
    } else {
        match member.fields.get(&condition.field_key) {
            Some(v) => v.clone(),
            None => return false,
        }
    };
    
    compare_value(&value, &condition.operator, &condition.target_values)
}

fn compare_value(value: &Value, operator: &ComparisonOperator, targets: &[String]) -> bool {
    match (value, operator) {
        (Value::Number(n), ComparisonOperator::Equal) => {
            targets.iter().any(|t| t.parse::<i32>().ok() == Some(*n))
        }
        (Value::Number(n), ComparisonOperator::In) => {
            targets.iter().any(|t| t.parse::<i32>().ok() == Some(*n))
        }
        (Value::Number(n), ComparisonOperator::GreaterThan) => {
            targets.iter().any(|t| t.parse::<i32>().ok().map(|t| n > &t).unwrap_or(false))
        }
        (Value::Number(n), ComparisonOperator::GreaterThanOrEqual) => {
            targets.iter().any(|t| t.parse::<i32>().ok().map(|t| n >= &t).unwrap_or(false))
        }
        (Value::Number(n), ComparisonOperator::LessThan) => {
            targets.iter().any(|t| t.parse::<i32>().ok().map(|t| n < &t).unwrap_or(false))
        }
        (Value::String(s), ComparisonOperator::Equal) => targets.contains(s),
        (Value::String(s), ComparisonOperator::In) => targets.contains(s),
        (Value::Date(d), ComparisonOperator::GreaterThanOrEqual) => {
            targets.iter().any(|t| d >= t)
        }
        _ => false,
    }
}

fn evaluate_all_different(
    schedule: &Schedule,
    problem_data: &ProblemData,
    unique_item_field: &str,
    group_item_field: &str,
) -> u32 {
    let (unique_item, unique_field) = parse_item_field(unique_item_field);
    let (group_item, group_field) = parse_item_field(group_item_field);
    
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    
    for assignment in &schedule.assignments {
        let group_value = extract_field_value(assignment, problem_data, &group_item, &group_field);
        let unique_value = extract_field_value(assignment, problem_data, &unique_item, &unique_field);
        
        groups.entry(group_value).or_default().push(unique_value);
    }
    
    let mut violations = 0;
    for values in groups.values() {
        let unique_count = values.iter().collect::<HashSet<_>>().len();
        if unique_count < values.len() {
            violations += (values.len() - unique_count) as u32;
        }
    }
    
    violations
}

fn evaluate_cardinality(
    schedule: &Schedule,
    problem_data: &ProblemData,
    target_item_field: &str,
    max_count: u32,
    scope_conditions: &Option<Vec<Condition>>,
) -> u32 {
    let (target_item, target_field) = parse_item_field(target_item_field);
    
    let filtered: Vec<&Assignment> = if let Some(conditions) = scope_conditions {
        schedule.assignments.iter()
            .filter(|a| {
                conditions.iter().all(|c| evaluate_condition(c, a, problem_data))
            })
            .collect()
    } else {
        schedule.assignments.iter().collect()
    };
    
    let mut counts: HashMap<String, u32> = HashMap::new();
    for assignment in filtered {
        let value = extract_field_value(assignment, problem_data, &target_item, &target_field);
        *counts.entry(value).or_default() += 1;
    }
    
    let mut violations = 0;
    for count in counts.values() {
        if *count > max_count {
            violations += count - max_count;
        }
    }
    
    violations
}

fn evaluate_temporal_precedence(
    schedule: &Schedule,
    problem_data: &ProblemData,
    grouping_item_field: &str,
    first_conditions: &[Condition],
    second_conditions: &[Condition],
    temporal_relation: &ComparisonOperator,
    temporal_fields: &[String],
) -> u32 {
    let (group_item, group_field) = parse_item_field(grouping_item_field);
    
    let mut groups: HashMap<String, Vec<&Assignment>> = HashMap::new();
    for assignment in &schedule.assignments {
        let group_value = extract_field_value(assignment, problem_data, &group_item, &group_field);
        groups.entry(group_value).or_default().push(assignment);
    }
    
    let mut violations = 0;
    for group_assignments in groups.values() {
        let firsts: Vec<&&Assignment> = group_assignments.iter()
            .filter(|a| first_conditions.iter().all(|c| evaluate_condition(c, a, problem_data)))
            .collect();
        
        let seconds: Vec<&&Assignment> = group_assignments.iter()
            .filter(|a| second_conditions.iter().all(|c| evaluate_condition(c, a, problem_data)))
            .collect();
        
        for first in &firsts {
            for second in &seconds {
                if !check_temporal_relation(first, second, problem_data, temporal_relation, temporal_fields) {
                    violations += 1;
                }
            }
        }
    }
    
    violations
}

fn check_temporal_relation(
    first: &Assignment,
    second: &Assignment,
    problem_data: &ProblemData,
    relation: &ComparisonOperator,
    temporal_fields: &[String],
) -> bool {
    if temporal_fields.len() != 2 {
        return false;
    }
    
    // Assuming temporal fields come from TimeSlot
    let first_time_id = match first.resources.get("TimeSlot") {
        Some(&id) => id,
        None => return false,
    };
    
    let second_time_id = match second.resources.get("TimeSlot") {
        Some(&id) => id,
        None => return false,
    };
    
    let time_item = match problem_data.item_categories.get("TimeSlot") {
        Some(i) => i,
        None => return false,
    };
    
    let first_member = time_item.members.iter().find(|m| m.id == first_time_id);
    let second_member = time_item.members.iter().find(|m| m.id == second_time_id);
    
    if let (Some(fm), Some(sm)) = (first_member, second_member) {
        let first_val = fm.fields.get(&temporal_fields[0]);
        let second_val = sm.fields.get(&temporal_fields[1]);
        
        if let (Some(Value::Date(fv)), Some(Value::Date(sv))) = (first_val, second_val) {
            match relation {
                ComparisonOperator::Before => fv < sv,
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn parse_item_field(item_field: &str) -> (String, String) {
    let parts: Vec<&str> = item_field.split(':').collect();
    (parts[0].to_string(), parts[1].to_string())
}

fn extract_field_value(
    assignment: &Assignment,
    problem_data: &ProblemData,
    item_name: &str,
    field_key: &str,
) -> String {
    let member_id = if item_name == assignment.task_item_name {
        assignment.task_id
    } else {
        match assignment.resources.get(item_name) {
            Some(&id) => id,
            None => return String::new(),
        }
    };
    
    if field_key == "id" {
        return member_id.0.to_string();
    }
    
    let item = match problem_data.item_categories.get(item_name) {
        Some(i) => i,
        None => return String::new(),
    };
    
    let member = match item.members.iter().find(|m| m.id == member_id) {
        Some(m) => m,
        None => return String::new(),
    };
    
    match member.fields.get(field_key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Date(d)) => d.clone(),
        None => String::new(),
    }
}
