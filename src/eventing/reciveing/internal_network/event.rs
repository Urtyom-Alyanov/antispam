use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
	pub user_id: i64,
	pub secret: String,
	pub event_type: String,
	pub id: i32,
}
