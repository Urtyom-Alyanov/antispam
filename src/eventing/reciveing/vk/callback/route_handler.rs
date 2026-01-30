use crate::{
    entities::group,
    eventing::reciveing::vk::callback::callback_models::{VkCallback,VkWallPostNew,VkWallReplyNew,},
    state::application::AppState,
    processes::{wall_post_new::process_wall_post_new,wall_reply_new::process_wall_reply_new}
};

use axum::{Json, extract::State};
use sea_orm::EntityTrait;
use std::sync::Arc;

pub async fn handle_flood(
	State(state): State<Arc<AppState>>,
	Json(payload): Json<VkCallback>,
) -> Result<String, String> {
	// Получаем настройки группы или шлём нахуй
	let Some(group_cfg) = group::Entity::find_by_id(payload.group_id)
		.one(&state.db)
		.await
		.unwrap_or(None)
	else {
		return Err("404".to_string());
	};

	if let Some(secret_db) = &group_cfg.secret {
		if payload.secret.as_ref() != Some(secret_db) {
			return Err("404".to_string());
		}
	}

	if !group_cfg.is_active {
		return Err("404".to_string());
	}

	match payload.event_id.as_str() {
		"confirmation" => Ok(group_cfg.confirmation_token.clone()),
		"wall_post_new" => {
			if let Ok(data) = serde_json::from_value::<VkWallPostNew>(payload.object) {
				process_wall_post_new(state, group_cfg.clone(), data).await;
				return Ok("ok".to_string());
			}
			return Err("500".to_string());
		}
		"wall_reply_new" => {
			if let Ok(data) = serde_json::from_value::<VkWallReplyNew>(payload.object) {
				process_wall_reply_new(state, group_cfg.clone(), data).await;
				return Ok("ok".to_string());
			}
			return Err("500".to_string());
		}
		_ => return Ok("ok".to_string()),
	}
}
