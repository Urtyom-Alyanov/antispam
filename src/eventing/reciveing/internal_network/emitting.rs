use std::time::Duration;

use log::error;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{entities::connection, eventing::reciveing::internal_network::event::Event, state::application::AppError};

pub async fn emit_all_connections(db: &DatabaseConnection, user_id: i64) -> Result<(), AppError> {
	let client = reqwest::Client::new();

	let connections = connection::Entity::find().all(db).await?;

	let requests = connections
		.into_iter()
		.filter(|connection| connection.secret_out.is_some())
		.map(|connection| {
			let client = client.clone();
			let event = Event {
				user_id,
				secret: connection.secret_out.unwrap(),
				event_type: "user_ban".to_string(),
				id: connection.id as i32,
			};

			let hostname = connection.hostname.clone();

			async move {
				client
					.post(hostname + "/internal/listen/" + &event.id.to_string())
					.json(&event)
					.timeout(Duration::from_secs(10))
					.send()
					.await
			}
		});

	let responses = futures::future::join_all(requests).await;

	for response in responses {
		if let Err(err) = response {
			error!("Failed to emit event: {}", err);
		}
	}

	Ok(())
}
