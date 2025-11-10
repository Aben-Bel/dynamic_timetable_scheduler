use crate::domain::*;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct SimulatedAnnealing {
    pub initial_temperature: f64,
    pub cooling_rate: f64,
    pub max_iterations: u32,
}

impl SimulatedAnnealing {
    pub fn new(initial_temperature: f64, cooling_rate: f64, max_iterations: u32) -> Self {
        Self {
            initial_temperature,
            cooling_rate,
            max_iterations,
        }
    }

    pub fn solve(
        &self,
        problem_data: &ProblemData,
        constraints: &[Constraint],
        initial_schedule: Schedule,
    ) -> Schedule {
        let mut current = initial_schedule;
        let mut best = current.clone();
        let mut current_cost = super::constraint_evaluator::evaluate_schedule(&current, problem_data, constraints);
        let mut best_cost = current_cost;

        let mut temperature = self.initial_temperature;
        let mut rng = rand::thread_rng();

        for iteration in 0..self.max_iterations {
            let neighbor = self.get_neighbor(&current, problem_data, &mut rng);
            let neighbor_cost = super::constraint_evaluator::evaluate_schedule(&neighbor, problem_data, constraints);

            let delta = neighbor_cost as i32 - current_cost as i32;
            if delta < 0 || self.should_accept(delta as f64, temperature, &mut rng) {
                current = neighbor;
                current_cost = neighbor_cost;

                if current_cost < best_cost {
                    best = current.clone();
                    best_cost = current_cost;
                }
            }

            temperature *= self.cooling_rate;

            if iteration % 100 == 0 {
                println!("Iteration {}: current_cost={}, best_cost={}", iteration, current_cost, best_cost);
            }
        }

        println!("Final best cost: {}", best_cost);
        best
    }

    fn should_accept(&self, delta: f64, temperature: f64, rng: &mut impl Rng) -> bool {
        if temperature < 1e-10 {
            return false;
        }
        let probability = (-delta / temperature).exp();
        rng.gen::<f64>() < probability
    }

    fn get_neighbor(&self, schedule: &Schedule, problem_data: &ProblemData, rng: &mut impl Rng) -> Schedule {
        let mut new_schedule = schedule.clone();

        if new_schedule.assignments.is_empty() {
            return new_schedule;
        }

        let operation = rng.gen::<f64>();

        if operation < 0.7 {
            // Move: reassign resources
            let idx = rng.gen_range(0..new_schedule.assignments.len());
            let assignment = &mut new_schedule.assignments[idx];
            
            // Pick random resource type to change
            let resource_names: Vec<String> = assignment.resources.keys().cloned().collect();
            if let Some(resource_name) = resource_names.choose(rng) {
                if let Some(item) = problem_data.item_categories.get(resource_name) {
                    if let Some(new_member) = item.members.choose(rng) {
                        assignment.resources.insert(resource_name.clone(), new_member.id);
                    }
                }
            }
} else {
    // Swap: exchange resources between two assignments
    if new_schedule.assignments.len() > 1 {
        let idx1 = rng.gen_range(0..new_schedule.assignments.len());
        let idx2 = rng.gen_range(0..new_schedule.assignments.len());
        
        if idx1 != idx2 {
            let resource_names: Vec<String> = new_schedule.assignments[idx1].resources.keys().cloned().collect();
            if let Some(resource_name) = resource_names.choose(rng) {
                // Extract values first to avoid borrow conflicts
                let r1 = new_schedule.assignments[idx1].resources.get(resource_name).copied();
                let r2 = new_schedule.assignments[idx2].resources.get(resource_name).copied();
                
                if let (Some(id1), Some(id2)) = (r1, r2) {
                    new_schedule.assignments[idx1].resources.insert(resource_name.clone(), id2);
                    new_schedule.assignments[idx2].resources.insert(resource_name.clone(), id1);
                }
            }
        }
    }
}

        new_schedule
    }
}
