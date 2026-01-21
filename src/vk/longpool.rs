use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::vk::vk_response::VkResponse;

#[derive(Deserialize, Serialize)]
pub struct VkLongPoolEventUpdate {
	#[serde(rename = "type")]
	pub event_type: String,
	pub event_id: String,
	pub v: String,
	pub object: serde_json::Value,
	pub group_id: u64,
}

pub struct VkLongpoolEvent {
	pub ts: u32,
	pub updates: Vec<serde_json::Value>,
	pub failed: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LongPoolServerResponse {
	pub server: String,
	pub key: String,
	pub ts: String,
}

pub struct VkLongPoolState {
	pub client: reqwest::Client,

	pub server: String,
	pub key: String,
	pub last_ts: u32,
	pub group_id: u64,
}

impl VkLongPoolState {
	pub async fn new(access_token: &str, group_id: &u64) -> Self {
		let client = reqwest::Client::new();

		let info = Self::get_new_server(client.clone(), access_token, group_id)
			.await
			.unwrap();

		Self {
			client: client.to_owned(),
			group_id: group_id.to_owned(),
			key: info.key,
			server: info.server,
			last_ts: info.ts.parse::<u32>().unwrap(),
		}
	}

	pub async fn get_new_server(
		client: reqwest::Client,
		access_token: &str,
		group_id: &u64,
	) -> Result<LongPoolServerResponse, reqwest::Error> {
		let method = "groups.getLongPollServer";

		let params = vec![
			("access_token", access_token.to_owned()),
			("group_id", group_id.to_string()),
			("v", "5.199".to_owned()),
		];

		let client_json_response = client
			.post(format!("https://api.vk.com/method/{}", method))
			.json(&params)
			.send()
			.await?
			.json::<VkResponse<LongPoolServerResponse>>()
			.await?;

		return Ok(client_json_response.response);
	}

	async fn pooling(&self) -> Result<VkLongPoolEventUpdate, reqwest::Error> {
		let waiting_secs = 90;

		let mut params = vec![
			("key", self.key.to_owned()),
			("ts", self.last_ts.to_string()),
			("wait", waiting_secs.to_string()),
		];

		self
			.client
			.get(self.server)
			.timeout(Duration::from_secs(waiting_secs))
			.send();
	}
}
