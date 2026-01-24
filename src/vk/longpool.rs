use std::time::Duration;

use reqwest::Url;
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

#[derive(Deserialize)]
struct VkLongpoolEvent {
	pub ts: u32,
	pub updates: Vec<VkLongPoolEventUpdate>,
	pub failed: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LongPoolServerResponse {
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

	access_token: String,
}

impl VkLongPoolState {
	pub async fn new(access_token: &str, group_id: &u64) -> Self {
		let client = reqwest::Client::new();

		let info = Self::get_new_server(&client, access_token, group_id)
			.await
			.unwrap();

		Self {
			client: client.to_owned(),
			group_id: group_id.to_owned(),
			key: info.key,
			server: info.server,
			last_ts: info.ts.parse::<u32>().unwrap(),
			access_token: access_token.to_owned(),
		}
	}

	async fn get_new_server(
		client: &reqwest::Client,
		access_token: &str,
		group_id: &u64,
	) -> Result<LongPoolServerResponse, reqwest::Error> {
		let method = "groups.getLongPollServer";

		let mut url = Url::parse(&format!("https://api.vk.com/method/{}", method)).unwrap();

		url
			.query_pairs_mut()
			.append_pair("access_token", access_token)
			.append_pair("group_id", &group_id.to_string())
			.append_pair("v", "5.199");

		let client_json_response = client
			.post(url)
			.send()
			.await?
			.json::<VkResponse<LongPoolServerResponse>>()
			.await?;

		return Ok(client_json_response.response);
	}

	pub async fn pool(&mut self) -> Result<Vec<VkLongPoolEventUpdate>, reqwest::Error> {
		let waiting_secs = 90;

		let mut url = Url::parse(&self.server).expect("Invalid server URL");
		url
			.query_pairs_mut()
			.append_pair("key", &self.key)
			.append_pair("ts", &self.last_ts.to_string())
			.append_pair("wait", &waiting_secs.to_string())
			.append_pair("act", "a_check");

		let response = self
			.client
			.get(url)
			.timeout(Duration::from_secs(waiting_secs + 5))
			.send()
			.await?
			.json::<VkLongpoolEvent>()
			.await?;

		if let Some(failed) = response.failed {
			match failed {
				1 => self.last_ts = response.ts,
				2 | 3 => {
					let new_info =
						Self::get_new_server(&self.client, &self.access_token, &self.group_id).await?;
					self.key = new_info.key;
					self.server = new_info.server;
					if failed == 3 {
						self.last_ts = new_info.ts.parse::<u32>().unwrap();
					}
				}
				_ => {}
			}
		}

		self.last_ts = response.ts;

		Ok(response.updates)
	}
}
