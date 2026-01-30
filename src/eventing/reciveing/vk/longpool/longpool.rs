use std::time::Duration;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::api::vk::vk_api::VkApi;

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
	pub server: String,
	pub key: String,
	pub last_ts: u32,
	pub group_id: u64,

	api_client: VkApi,
	http_client: Client
}

#[derive(Serialize)]
struct VkLongPoolGetServerOptions {
    pub group_id: u64,
}

impl VkLongPoolState {
	pub async fn new(access_token: &str, group_id: u64) -> Result<Self, reqwest::Error> {
		let api_client = VkApi::new(access_token, None);

		let info = Self::get_new_server(&api_client, group_id).await?;

		Ok(Self {
			group_id: group_id.to_owned(),
			key: info.key,
			server: info.server,
			last_ts: info.ts.parse::<u32>().unwrap(),
			api_client: api_client,
			http_client: Client::new(),
		})
	}

	async fn get_new_server(
	    api_client: &VkApi,
		group_id: u64,
	) -> Result<LongPoolServerResponse, reqwest::Error> {
		return api_client.resolve::<_, LongPoolServerResponse>(
    		"groups.getLongPollServer",
    		VkLongPoolGetServerOptions { group_id: group_id }
		).await;
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
			.http_client
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
						Self::get_new_server(&self.api_client, self.group_id).await?;
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
