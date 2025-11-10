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
        // ---------- Numbers ----------
        (Value::Number(n), op) => {
            // parse all targets as i32 (ignore ones that fail to parse)
            let parsed: Vec<i32> = targets
                .iter()
                .filter_map(|t| t.parse::<i32>().ok())
                .collect();

            if parsed.is_empty() {
                return false;
            }

            match op {
                ComparisonOperator::Equal => parsed.iter().any(|t| n == t),
                ComparisonOperator::NotEqual => parsed.iter().all(|t| n != t),
                ComparisonOperator::In => parsed.iter().any(|t| n == t),
                ComparisonOperator::NotIn => parsed.iter().all(|t| n != t),

                ComparisonOperator::GreaterThan => parsed.iter().any(|t| n > t),
                ComparisonOperator::GreaterThanOrEqual => parsed.iter().any(|t| n >= t),
                ComparisonOperator::LessThan => parsed.iter().any(|t| n < t),
                ComparisonOperator::LessThanOrEqual => parsed.iter().any(|t| n <= t),

                // Before / After / Overlap / NoOverlap don't have a useful numeric meaning here
                _ => false,
            }
        }

        // ---------- Strings ----------
        (Value::String(s), op) => {
            if targets.is_empty() {
                return false;
            }

            match op {
                ComparisonOperator::Equal => targets.iter().any(|t| s == t),
                ComparisonOperator::NotEqual => targets.iter().all(|t| s != t),
                ComparisonOperator::In => targets.iter().any(|t| s == t),
                ComparisonOperator::NotIn => targets.iter().all(|t| s != t),

                // Lexicographic comparisons
                ComparisonOperator::GreaterThan => targets.iter().any(|t| s > t),
                ComparisonOperator::GreaterThanOrEqual => targets.iter().any(|t| s >= t),
                ComparisonOperator::LessThan => targets.iter().any(|t| s < t),
                ComparisonOperator::LessThanOrEqual => targets.iter().any(|t| s <= t),

                // Temporal-ish operators on plain strings: treat as lexicographic dates/times if caller uses them
                ComparisonOperator::Before => targets.iter().any(|t| s < t),
                ComparisonOperator::After => targets.iter().any(|t| s > t),

                // Overlap / NoOverlap on strings: treat targets as a range [min, max]
                ComparisonOperator::Overlap => {
                    let min = targets.iter().min().unwrap();
                    let max = targets.iter().max().unwrap();
                    s >= min && s <= max
                }
                ComparisonOperator::NoOverlap => {
                    let min = targets.iter().min().unwrap();
                    let max = targets.iter().max().unwrap();
                    s < min || s > max
                }
            }
        }

        // ---------- Dates (stored as String, e.g. "13:30" or ISO) ----------
        (Value::Date(d), op) => {
            if targets.is_empty() {
                return false;
            }

            match op {
                ComparisonOperator::Equal => targets.iter().any(|t| d == t),
                ComparisonOperator::NotEqual => targets.iter().all(|t| d != t),
                ComparisonOperator::In => targets.iter().any(|t| d == t),
                ComparisonOperator::NotIn => targets.iter().all(|t| d != t),

                ComparisonOperator::GreaterThan => targets.iter().any(|t| d > t),
                ComparisonOperator::GreaterThanOrEqual => targets.iter().any(|t| d >= t),
                ComparisonOperator::LessThan => targets.iter().any(|t| d < t),
                ComparisonOperator::LessThanOrEqual => targets.iter().any(|t| d <= t),

                // For scalar dates, Before / After are just < and >
                ComparisonOperator::Before => targets.iter().any(|t| d < t),
                ComparisonOperator::After => targets.iter().any(|t| d > t),

                // Overlap / NoOverlap: treat targets as an interval [min, max]
                ComparisonOperator::Overlap => {
                    let min = targets.iter().min().unwrap();
                    let max = targets.iter().max().unwrap();
                    d >= min && d <= max
                }
                ComparisonOperator::NoOverlap => {
                    let min = targets.iter().min().unwrap();
                    let max = targets.iter().max().unwrap();
                    d < min || d > max
                }
            }
        }

        // Any other combination is unsupported
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
    if temporal_fields.is_empty() || temporal_fields.len() > 2 {
        return false;
    }

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

    let first_member = match time_item.members.iter().find(|m| m.id == first_time_id) {
        Some(m) => m,
        None => return false,
    };
    let second_member = match time_item.members.iter().find(|m| m.id == second_time_id) {
        Some(m) => m,
        None => return false,
    };

    // Extract start/end strings
    let (first_start, first_end, second_start, second_end) = if temporal_fields.len() == 1 {
        let field = &temporal_fields[0];
        let fs = match first_member.fields.get(field) {
            Some(Value::Date(v)) => v.as_str(),
            _ => return false,
        };
        let ss = match second_member.fields.get(field) {
            Some(Value::Date(v)) => v.as_str(),
            _ => return false,
        };
        (fs, fs, ss, ss)
    } else {
        let start_field = &temporal_fields[0];
        let end_field = &temporal_fields[1];

        let fs = match first_member.fields.get(start_field) {
            Some(Value::Date(v)) => v.as_str(),
            _ => return false,
        };
        let fe = match first_member.fields.get(end_field) {
            Some(Value::Date(v)) => v.as_str(),
            _ => return false,
        };
        let ss = match second_member.fields.get(start_field) {
            Some(Value::Date(v)) => v.as_str(),
            _ => return false,
        };
        let se = match second_member.fields.get(end_field) {
            Some(Value::Date(v)) => v.as_str(),
            _ => return false,
        };

        (fs, fe, ss, se)
    };

    match relation {
        ComparisonOperator::Before => first_end < second_start,
        ComparisonOperator::After => first_start > second_end,
        ComparisonOperator::Overlap => first_start < second_end && second_start < first_end,
        ComparisonOperator::NoOverlap => !(first_start < second_end && second_start < first_end),

        ComparisonOperator::Equal => first_start == second_start,
        ComparisonOperator::NotEqual => first_start != second_start,
        ComparisonOperator::GreaterThan => first_start > second_start,
        ComparisonOperator::GreaterThanOrEqual => first_start >= second_start,
        ComparisonOperator::LessThan => first_start < second_start,
        ComparisonOperator::LessThanOrEqual => first_start <= second_start,

        _ => false,
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
