use std::{collections::HashMap, sync::Arc};
use scheduling_optimizer::{app_state::AppState, domain::*};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let problem_data = create_sample_problem();
    let constraints = create_sample_constraints();

    let app_state = AppState::new(
        Arc::new(RwLock::new(problem_data)),
        Arc::new(RwLock::new(constraints)),
    );

    let app = scheduling_optimizer::Application::build(app_state, "127.0.0.1:3000").await?;
    app.run().await?;

    Ok(())
}


fn create_sample_problem() -> ProblemData {
    let mut item_categories = HashMap::new();

    // Course Item
    let course_schema = Schema {
        definitions: HashMap::from([
            (
                "name".to_string(),
                FieldSchema {
                    field_name: "name".to_string(),
                    field_type: FieldType::Text,
                    is_required: true,
                },
            ),
            (
                "duration".to_string(),
                FieldSchema {
                    field_name: "duration".to_string(),
                    field_type: FieldType::Integer,
                    is_required: true,
                },
            ),
        ]),
    };

    let courses = vec![
        Member {
            id: ItemId(1),
            fields: HashMap::from([
                (
                    "name".to_string(),
                    Value::String("Photogrammetric CV".to_string()),
                ),
                ("duration".to_string(), Value::Number(90)),
            ]),
        },
        Member {
            id: ItemId(2),
            fields: HashMap::from([
                (
                    "name".to_string(),
                    Value::String("Machine Learning".to_string()),
                ),
                ("duration".to_string(), Value::Number(90)),
            ]),
        },
        Member {
            id: ItemId(3),
            fields: HashMap::from([
                (
                    "name".to_string(),
                    Value::String("Virtual Reality".to_string()),
                ),
                ("duration".to_string(), Value::Number(90)),
            ]),
        },
        Member {
            id: ItemId(4),
            fields: HashMap::from([
                ("name".to_string(), Value::String("HCI Theory".to_string())),
                ("duration".to_string(), Value::Number(90)),
            ]),
        },
        Member {
            id: ItemId(5),
            fields: HashMap::from([
                (
                    "name".to_string(),
                    Value::String("Computer Vision".to_string()),
                ),
                ("duration".to_string(), Value::Number(90)),
            ]),
        },
    ];

    item_categories.insert(
        "Course".to_string(),
        Item {
            name: "Course".to_string(),
            item_set_type: SetType::B_Set,
            members: courses,
            schema: course_schema,
        },
    );

    // Room Item
    let room_schema = Schema {
        definitions: HashMap::from([
            (
                "name".to_string(),
                FieldSchema {
                    field_name: "name".to_string(),
                    field_type: FieldType::Text,
                    is_required: true,
                },
            ),
            (
                "capacity".to_string(),
                FieldSchema {
                    field_name: "capacity".to_string(),
                    field_type: FieldType::Integer,
                    is_required: true,
                },
            ),
        ]),
    };

    let rooms = vec![
        Member {
            id: ItemId(1),
            fields: HashMap::from([
                ("name".to_string(), Value::String("B11".to_string())),
                ("capacity".to_string(), Value::Number(50)),
            ]),
        },
        Member {
            id: ItemId(2),
            fields: HashMap::from([
                ("name".to_string(), Value::String("SR_A".to_string())),
                ("capacity".to_string(), Value::Number(30)),
            ]),
        },
        Member {
            id: ItemId(3),
            fields: HashMap::from([
                ("name".to_string(), Value::String("SR_H".to_string())),
                ("capacity".to_string(), Value::Number(25)),
            ]),
        },
        Member {
            id: ItemId(4),
            fields: HashMap::from([
                ("name".to_string(), Value::String("LH_HK7".to_string())),
                ("capacity".to_string(), Value::Number(100)),
            ]),
        },
    ];

    // TimeSlot Item
    let time_schema = Schema {
        definitions: HashMap::from([
            (
                "day".to_string(),
                FieldSchema {
                    field_name: "day".to_string(),
                    field_type: FieldType::Text,
                    is_required: true,
                },
            ),
            (
                "start".to_string(),
                FieldSchema {
                    field_name: "start".to_string(),
                    field_type: FieldType::DateTime,
                    is_required: true,
                },
            ),
            (
                "end".to_string(),
                FieldSchema {
                    field_name: "end".to_string(),
                    field_type: FieldType::DateTime,
                    is_required: true,
                },
            ),
        ]),
    };

    item_categories.insert(
        "Room".to_string(),
        Item {
            name: "Room".to_string(),
            item_set_type: SetType::E_Set,
            members: rooms,
            schema: room_schema,
        },
    );

    // TimeSlot Item
    let time_slots = vec![
        Member {
            id: ItemId(1),
            fields: HashMap::from([
                ("day".to_string(), Value::String("Monday".to_string())),
                ("start".to_string(), Value::Date("08:00".to_string())),
                ("end".to_string(), Value::Date("09:30".to_string())),
            ]),
        },
        Member {
            id: ItemId(2),
            fields: HashMap::from([
                ("day".to_string(), Value::String("Monday".to_string())),
                ("start".to_string(), Value::Date("09:45".to_string())),
                ("end".to_string(), Value::Date("11:15".to_string())),
            ]),
        },
        // 12:00-13:30 is LUNCH (no slot)
        Member {
            id: ItemId(3),
            fields: HashMap::from([
                ("day".to_string(), Value::String("Monday".to_string())),
                ("start".to_string(), Value::Date("13:30".to_string())),
                ("end".to_string(), Value::Date("15:00".to_string())),
            ]),
        },
        Member {
            id: ItemId(4),
            fields: HashMap::from([
                ("day".to_string(), Value::String("Tuesday".to_string())),
                ("start".to_string(), Value::Date("09:45".to_string())),
                ("end".to_string(), Value::Date("11:15".to_string())),
            ]),
        },
        Member {
            id: ItemId(5),
            fields: HashMap::from([
                ("day".to_string(), Value::String("Wednesday".to_string())),
                ("start".to_string(), Value::Date("13:30".to_string())),
                ("end".to_string(), Value::Date("15:00".to_string())),
            ]),
        },
        Member {
            id: ItemId(6),
            fields: HashMap::from([
                ("day".to_string(), Value::String("Thursday".to_string())),
                ("start".to_string(), Value::Date("09:45".to_string())),
                ("end".to_string(), Value::Date("11:15".to_string())),
            ]),
        },
        Member {
            id: ItemId(7),
            fields: HashMap::from([
                ("day".to_string(), Value::String("Friday".to_string())),
                ("start".to_string(), Value::Date("15:15".to_string())),
                ("end".to_string(), Value::Date("16:45".to_string())),
            ]),
        },
    ];


    item_categories.insert(
        "TimeSlot".to_string(),
        Item {
            name: "TimeSlot".to_string(),
            item_set_type: SetType::E_Set,
            members: time_slots,
            schema: time_schema,
        },
    );

    // Lecturer Item
    let lecturer_schema = Schema {
        definitions: HashMap::from([(
            "name".to_string(),
            FieldSchema {
                field_name: "name".to_string(),
                field_type: FieldType::Text,
                is_required: true,
            },
        )]),
    };

    let lecturers = vec![
        Member {
            id: ItemId(1),
            fields: HashMap::from([(
                "name".to_string(),
                Value::String("Prof. Rodehorst".to_string()),
            )]),
        },
        Member {
            id: ItemId(2),
            fields: HashMap::from([("name".to_string(), Value::String("Prof. Stein".to_string()))]),
        },
        Member {
            id: ItemId(3),
            fields: HashMap::from([(
                "name".to_string(),
                Value::String("Prof. Fröhlich".to_string()),
            )]),
        },
        Member {
            id: ItemId(4),
            fields: HashMap::from([(
                "name".to_string(),
                Value::String("Prof. Hornecker".to_string()),
            )]),
        },
    ];

    item_categories.insert(
        "Lecturer".to_string(),
        Item {
            name: "Lecturer".to_string(),
            item_set_type: SetType::E_Set,
            members: lecturers,
            schema: lecturer_schema,
        },
    );

    ProblemData { item_categories }
}

fn create_sample_constraints() -> Vec<Constraint> {
    vec![
        // === HARD CONSTRAINTS (weight 100+) ===

        // No double-booking
        Constraint {
            name: "No Room Conflicts".to_string(),
            weight: 100,
            rule: ConstraintRule::GlobalAllDifferent {
                unique_item_field: "Room:id".to_string(),
                group_item_field: "TimeSlot:id".to_string(),
            },
        },
        Constraint {
            name: "No Lecturer Conflicts".to_string(),
            weight: 100,
            rule: ConstraintRule::GlobalAllDifferent {
                unique_item_field: "Lecturer:id".to_string(),
                group_item_field: "TimeSlot:id".to_string(),
            },
        },
        // Lunch break (12:00-13:30 protected)
        Constraint {
            name: "Mandatory Lunch Break".to_string(),
            weight: 150,
            rule: ConstraintRule::MultiAssignmentCheck {
                conditions: vec![Condition {
                    item_name: "TimeSlot".to_string(),
                    field_key: "start".to_string(),
                    operator: ComparisonOperator::In,
                    target_values: vec![
                        "12:00".to_string(),
                        "12:30".to_string(),
                        "13:00".to_string(),
                    ],
                }],
                logical_op: LogicalOperator::Or,
                mode: ConstraintMode::Forbid,
            },
        },
        // Room capacity (assuming courses have enrollment field)
        Constraint {
            name: "Room Must Fit Students".to_string(),
            weight: 100,
            rule: ConstraintRule::MultiAssignmentCheck {
                conditions: vec![Condition {
                    item_name: "Room".to_string(),
                    field_key: "capacity".to_string(),
                    operator: ComparisonOperator::LessThan,
                    target_values: vec!["40".to_string()],
                }],
                logical_op: LogicalOperator::And,
                mode: ConstraintMode::Forbid,
            },
        },
        // === SOFT CONSTRAINTS ===

        // Typical lecturer load: 1-2 courses per semester (STRICT)
        Constraint {
            name: "Max 2 Courses Per Lecturer".to_string(),
            weight: 80,
            rule: ConstraintRule::GlobalCardinality {
                target_item_field: "Lecturer:id".to_string(),
                max_count: 2,
                scope_conditions: None,
            },
        },
        // Prefer compact schedule - avoid late evenings (after 18:00)
        Constraint {
            name: "No Late Evening Classes".to_string(),
            weight: 60,
            rule: ConstraintRule::MultiAssignmentCheck {
                conditions: vec![Condition {
                    item_name: "TimeSlot".to_string(),
                    field_key: "start".to_string(),
                    operator: ComparisonOperator::GreaterThanOrEqual,
                    target_values: vec!["18:00".to_string()],
                }],
                logical_op: LogicalOperator::And,
                mode: ConstraintMode::Forbid,
            },
        },
        // Friday afternoon soft preference
        Constraint {
            name: "Avoid Friday Afternoon".to_string(),
            weight: 25,
            rule: ConstraintRule::MultiAssignmentCheck {
                conditions: vec![
                    Condition {
                        item_name: "TimeSlot".to_string(),
                        field_key: "day".to_string(),
                        operator: ComparisonOperator::Equal,
                        target_values: vec!["Friday".to_string()],
                    },
                    Condition {
                        item_name: "TimeSlot".to_string(),
                        field_key: "start".to_string(),
                        operator: ComparisonOperator::GreaterThanOrEqual,
                        target_values: vec!["13:00".to_string()],
                    },
                ],
                logical_op: LogicalOperator::And,
                mode: ConstraintMode::Forbid,
            },
        },
        // Prefer morning slots for lectures (before 13:00)
        Constraint {
            name: "Prefer Morning Teaching".to_string(),
            weight: 15,
            rule: ConstraintRule::MultiAssignmentCheck {
                conditions: vec![Condition {
                    item_name: "TimeSlot".to_string(),
                    field_key: "start".to_string(),
                    operator: ComparisonOperator::GreaterThanOrEqual,
                    target_values: vec!["15:00".to_string()],
                }],
                logical_op: LogicalOperator::And,
                mode: ConstraintMode::Forbid,
            },
        },
        // Full-time professors need one free afternoon for research/admin
        // (This would need professor type field - example shown)
        Constraint {
            name: "Professor Research Time".to_string(),
            weight: 40,
            rule: ConstraintRule::GlobalCardinality {
                target_item_field: "Lecturer:id".to_string(),
                max_count: 3, // Max 3 afternoon slots per lecturer
                scope_conditions: Some(vec![Condition {
                    item_name: "TimeSlot".to_string(),
                    field_key: "start".to_string(),
                    operator: ComparisonOperator::GreaterThanOrEqual,
                    target_values: vec!["13:30".to_string()],
                }]),
            },
        },
    ]
}
