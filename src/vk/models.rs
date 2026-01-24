use serde::Deserialize;

#[derive(Deserialize)]
pub struct VkCallback {
	#[serde(rename = "type")]
	pub event_type: String,
	pub group_id: u64,
	pub object: serde_json::Value, // Вложенная структура
	pub v: String,                 // Версия API
	pub event_id: String,          // Идентификатор события (если будут дубликаты)
	pub secret: Option<String>,    // Секретный ключ для проверки, что это реально ВК
}

#[derive(Deserialize)]
pub struct VkWallReplyNew {
	pub id: i64,
	pub owner_id: i64, // Владелец стены
	pub from_id: i64,
}

#[derive(Deserialize)]
pub struct VkWallPostNew {
	pub id: i64,
	pub owner_id: i64, // Владелец стены
	pub from_id: i64,
}

#[derive(Deserialize, Debug)]
pub struct VkMessageNew {
	pub message: MessageData, // В message_new данные лежат в поле message
}

#[derive(Deserialize, Debug)]
pub struct MessageData {
	pub id: i64,
	pub from_id: i64,
	pub text: String,
	pub date: i64,
	pub peer_id: i64,
}
