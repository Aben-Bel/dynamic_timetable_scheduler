use axum::{Router, response::{IntoResponse, Redirect}, routing::{get, post, put}};
use tower_http::{cors::CorsLayer, services::fs::ServeDir};

pub mod domain;
pub mod application;
pub mod app_state;
pub mod api_error;
pub mod routes;

use app_state::AppState;
use routes::*;

pub struct Application {
    router: Router,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let cors = CorsLayer::permissive();
        
        let router = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(root))
        // items
        .route("/items", post(create_item))
        .route("/items", get(list_items))
        .route("/items/{item_name}", put(update_item).delete(delete_item))
        // members
        .route("/items/{item_name}/members", post(add_member))
        .route("/items/{item_name}/members", get(list_members))
        .route("/items/{item_name}/members/{member_id}", put(update_member).delete(delete_member))
        // constraints
        .route("/constraints", post(create_constraint))
        .route("/constraints", get(list_constraints))
        .route("/constraints/{name}", put(update_constraint).delete(delete_constraint))
        // solver
        .route("/solve", post(solve))
            .layer(cors)
            .with_state(app_state);


        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();

        Ok(Self { router, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("🚀 Server running at http://{}", &self.address);
        println!("📱 Open http://127.0.0.1:3000 in your browser");
        let listener = tokio::net::TcpListener::bind(&self.address).await?;
        axum::serve(listener, self.router).await
    }
}

async fn root() -> impl IntoResponse {
    // Redirects the user's browser from / to /public/index.html
    Redirect::to("/public/index.html")
}