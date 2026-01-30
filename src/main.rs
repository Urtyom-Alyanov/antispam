mod api;
mod application_state;
mod connections;
mod entities;
mod eventing;
mod flood_state;
mod handle_flood;
mod processes;
mod setup_database;

use crate::{
	application_state::AppState, connections::listening::listening_connections, entities::group,
	flood_state::FloodState, handle_flood::handle_flood, setup_database::setup_database,
	vk::longpool::VkLongPoolState,
};

use std::{sync::Arc, time::Duration};

use axum::{Router, routing::post};
use sea_orm::EntityTrait;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let data = Arc::new(AppState {
		db: setup_database().await,
		protector: FloodState::new(),
	});

	let mode = std::env::args()
		.nth(1)
		.expect("Usage: <mode: callback|longpool|hybrid>");

	let data_for_http = Arc::clone(&data);
	let data_for_lp = Arc::clone(&data);

	match mode.as_str() {
		"vk_callback" => run_http_server(data_for_http).await,
		"longpool" => run_longpool_service(data_for_lp).await,
		"hybrid" => {
			tokio::select! {
					_ = run_http_server(data_for_http) => {},
					_ = run_longpool_service(data_for_lp) => {},
			}
		}
		_ => panic!("Unknown mode: {}", mode),
	}
}
