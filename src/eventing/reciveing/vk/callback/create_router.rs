use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    eventing::reciveing::vk::callback::route_handler::handle_flood,
    state::application::AppState
};

pub fn create_vk_callback_router() -> Router<Arc<AppState>> {
	return Router::new()
		.route("/", post(handle_flood));
}
