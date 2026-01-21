use std::sync::Arc;

use chrono::TimeDelta;
use sea_orm::EntityTrait;

use crate::{
	AppState,
	entities::{group, setting},
	flood_state::ContentType,
};

pub async fn check_and_push_msgs_state(
	state: Arc<AppState>,
	group: group::Model,
	content_type: ContentType,
	user_id: i64,
	id: i64,
	owner_id: Option<i64>,
) -> bool {
	let Some(limit_seconds) = setting::Entity::find_by_id("LIMIT_SECONDS")
		.one(&state.db)
		.await
		.unwrap_or(None)
	else {
		panic!("LIMIT_SECONDS not found");
	};

	let Some(limit_count) = setting::Entity::find_by_id("LIMIT_COUNT")
		.one(&state.db)
		.await
		.unwrap_or(None)
	else {
		panic!("LIMIT_COUNT not found");
	};

	let is_spam = state.protector.check_and_push_msgs(
		user_id,
		id,
		content_type,
		owner_id,
		TimeDelta::seconds(
			group
				.limit_secs
				.unwrap_or(limit_seconds.value.parse::<i32>().unwrap_or(10)) as i64,
		),
		group
			.limit_count
			.unwrap_or(limit_count.value.parse::<i32>().unwrap_or(20)) as usize,
	);

	return is_spam;
}
