use std::sync::Arc;

use axum::extract::{Json, Path, State};
use sea_orm::EntityTrait;

use crate::{AppState, connections::event::Event, entities::connection};

pub async fn listening_connections(
	Path(id): Path<Option<i32>>,
	State(state): State<Arc<AppState>>,
	Json(payload): Json<Event>,
) -> Result<String, String> {
	let Some(_connection) = connection::Entity::find_by_id(id.unwrap_or(payload.id))
		.one(&state.db)
		.await
		.unwrap_or(None)
	else {
		return Err("Connection not found".to_string());
	};

	return Ok("ok".to_string());
}
