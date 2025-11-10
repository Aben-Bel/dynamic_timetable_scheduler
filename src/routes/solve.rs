use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{api_error::ApiError, app_state::AppState, application::SimulatedAnnealing, domain::*};
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
pub struct SolveRequest {
    pub initial_temperature: f64,
    pub cooling_rate: f64,
    pub max_iterations: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SolveResponse {
    pub schedule: Schedule,
    pub final_cost: u32,
}

pub async fn solve(
    State(state): State<AppState>,
    Json(request): Json<SolveRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let problem_data = state.problem_data.read().await;
    let constraints = state.constraints.read().await;
    
    let initial_schedule = create_random_schedule(&problem_data);
    
    let solver = SimulatedAnnealing::new(
        request.initial_temperature,
        request.cooling_rate,
        request.max_iterations,
    );
    
    let best_schedule = solver.solve(&problem_data, &constraints, initial_schedule);
    let final_cost = crate::application::evaluate_schedule(&best_schedule, &problem_data, &constraints);
    
    Ok((StatusCode::OK, Json(SolveResponse {
        schedule: best_schedule,
        final_cost,
    })))
}

fn create_random_schedule(problem_data: &ProblemData) -> Schedule {
    let mut rng = rand::thread_rng();
    let mut assignments = Vec::new();

    if let Some(course_item) = problem_data.item_categories.values()
        .find(|item| item.item_set_type == SetType::B_Set) {
        
        let e_set_items: Vec<_> = problem_data.item_categories.values()
            .filter(|item| item.item_set_type == SetType::E_Set)
            .collect();

        for course in &course_item.members {
            let mut resources = std::collections::HashMap::new();
            
            for e_item in &e_set_items {
                if let Some(member) = e_item.members.choose(&mut rng) {
                    resources.insert(e_item.name.clone(), member.id);
                }
            }

            assignments.push(Assignment {
                task_id: course.id,
                task_item_name: course_item.name.clone(),
                resources,
            });
        }
    }

    Schedule::new(assignments)
}
