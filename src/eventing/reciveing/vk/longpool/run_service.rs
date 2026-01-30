use std::{sync::Arc, time::Duration};

use sea_orm::EntityTrait;

use crate::{entities::group, eventing::reciveing::vk::longpool::longpool::VkLongPoolState, state::application::AppState};

pub async fn run_longpool_service(state: Arc<AppState>) {
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
			let mut event_looper = VkLongPoolState::new(&token, id).await?;

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
