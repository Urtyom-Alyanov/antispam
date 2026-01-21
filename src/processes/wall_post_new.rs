use std::sync::Arc;

use crate::{
	AppState,
	entities::group,
	flood_state::ContentType,
	processes::functions::{
		check_and_push_msgs_state::check_and_push_msgs_state, process_delete::process_delete,
	},
	vk::models::VkWallPostNew,
};

pub async fn process_wall_post_new(
	state: Arc<AppState>,
	group: group::Model,
	payload: VkWallPostNew,
) {
	let is_spam = check_and_push_msgs_state(
		state.clone(),
		group.clone(),
		ContentType::WallPost,
		payload.from_id,
		payload.id,
		Some(payload.owner_id),
	)
	.await;

	if is_spam {
		process_delete(state, group, payload.from_id).await;
	}
}
