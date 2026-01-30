pub async fn run_vk_callback_server(state: Arc<AppState>) {
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
