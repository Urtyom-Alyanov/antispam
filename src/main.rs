mod application_state;
mod connections;
mod entities;
mod flood_state;
mod handle_flood;
mod processes;
mod setup_database;
mod vk;

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

async fn run_http_server(state: Arc<AppState>) {
	let app = Router::new()
		.route("/", post(handle_flood))
		.route("/connection/listening/:id", post(listening_connections))
		.with_state(state);

	let addr = "0.0.0.0:8080";
	let listener = tokio::net::TcpListener::bind(addr)
		.await
		.expect("Failed to bind");
	println!("🚀 Callback server running on {}", addr);
	axum::serve(listener, app).await.unwrap();
}

async fn run_longpool_service(state: Arc<AppState>) {
	let groups = group::Entity::find()
		.all(&state.db)
		.await
		.expect("Database Error");

	let mut handles = vec![];

	for group in groups {
		let token = group.token.clone();
		let id = group.id;

		let handle = tokio::spawn(async move {
			println!("📡 Starting LongPoll for group: {}", id);
			let mut event_looper = VkLongPoolState::new(&token, &id).await;

			loop {
				match event_looper.pool().await {
					Ok(updates) => {
						for update in updates {
							match update.event_type.as_str() {
								"message_new" => println!("New message in group {}", id),
								_ => println!("Event: {} in group {}", update.event_type, id),
							}
						}
					}
					Err(e) => {
						eprintln!("LP Error (group {}): {}. Retrying in 5s...", id, e);
						tokio::time::sleep(Duration::from_secs(5)).await;
					}
				}
			}
		});
		handles.push(handle);
	}

	futures::future::join_all(handles).await;
}
