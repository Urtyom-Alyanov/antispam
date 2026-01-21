mod application_state;
mod connections;
mod entities;
mod flood_state;
mod handle_flood;
mod processes;
mod setup_database;
mod vk;

use crate::{
	application_state::AppState, connections::listening::listening_connections,
	flood_state::FloodState, handle_flood::handle_flood, setup_database::setup_database,
};

use std::sync::Arc;

use axum::{Router, routing::post};

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let data = Arc::new(AppState {
		db: setup_database().await,
		protector: FloodState::new(),
	});

	let app = Router::new()
		.route("/", post(handle_flood))
		.route("/connection/listening/:id", post(listening_connections))
		.with_state(data);

	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}
