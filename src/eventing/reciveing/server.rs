use std::{collections::HashSet, sync::Arc};

use axum::{Router, routing::post};

use crate::{eventing::reciveing::{internal_network::listening::listening_connections, vk::callback::create_router::create_vk_callback_router}, state::application::AppState};

#[derive(PartialEq, Eq, Hash)]
pub enum ServerFlow {
    VkCallback,
    Internal
}

pub async fn start_server(state: Arc<AppState>, flows: HashSet<ServerFlow>) {
    if flows.len() <= 0 {
        return;
    }

    let mut app =  Router::new();

    if flows.contains(&ServerFlow::VkCallback) {
        let callback_router = create_vk_callback_router();
        app = app.nest("/vk/callback", callback_router);
        println!("🚀 Server listen vk callback events");
    }

    if flows.contains(&ServerFlow::Internal) {
        let internal_router = Router::new().route("/listen", post(listening_connections));
        app = app.nest("/internal", internal_router);
        println!("🚀 Server listen internal network");
    }

    let app = app.with_state(state);

	let addr = "0.0.0.0:8080";
	let listener = tokio::net::TcpListener::bind(addr)
		.await
		.expect("Failed to bind");
	println!("🚀 Server running on {}", addr);
	axum::serve(listener, app).await.unwrap();
}
