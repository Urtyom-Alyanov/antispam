use std::sync::Arc;

use rand::Rng;
use sea_orm::EntityTrait;

use crate::{
	application_state::AppState,
	connections::emitting::emit_all_connections,
	entities::{group, setting},
	vk::delete_vk_content::delete_vk_content,
};

pub async fn process_delete(state: Arc<AppState>, group: group::Model, user_id: i64) {
	let Some(standard_user_token) = setting::Entity::find_by_id("USER_TOKEN")
		.one(&state.db)
		.await
		.unwrap_or(None)
	else {
		panic!("USER_TOKEN not found");
	};

	let Some(min_delay_seconds) = setting::Entity::find_by_id("MIN_DELAY_SECONDS")
		.one(&state.db)
		.await
		.unwrap_or(None)
	else {
		panic!("MIN_DELAY_SECONDS not found");
	};

	let Some(max_delay_seconds) = setting::Entity::find_by_id("MAX_DELAY_SECONDS")
		.one(&state.db)
		.await
		.unwrap_or(None)
	else {
		panic!("MAX_DELAY_SECONDS not found");
	};

	let token = group
		.user_token
		.unwrap_or(standard_user_token.value.clone());

	let state = Arc::clone(&state);

	let mut rng = rand::rng();
	let delay = rng.random_range(
		min_delay_seconds.value.parse::<u64>().unwrap()
			..=max_delay_seconds.value.parse::<u64>().unwrap(),
	);

	if (!state.protector.is_processing(user_id)) {
		state.protector.set_processing(user_id, true);
		tokio::spawn(async move {
			tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
			emit_all_connections(&state.db, user_id)
				.await
				.expect("При уведомлении произошла ошибка!");
			let msgs = state.protector.get_history(user_id);

			for msg in msgs {
				delete_vk_content(&token, msg)
					.await
					.expect("При удалении произошла ошибка!");
			}

			state.protector.clear_history(user_id);
			state.protector.set_processing(user_id, false);
		});
	}
}
