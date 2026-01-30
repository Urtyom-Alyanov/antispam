use reqwest::{Client, Error, Url};
use serde::{Deserialize, Serialize};

use crate::api::vk::vk_api_response_wrapper::VkApiResponseWrapper;

#[derive(Debug, Deserialize, Serialize)]
struct VkApiResponseWrapper<T> {
	pub response: T,
}

pub struct VkApi {
	token: String,
	version: String,
	client: Client,
}

impl VkApi {
	pub fn new(token: &str, version: Option<&str>) -> Self {
		Self {
			token: token.to_owned(),
			version: version.unwrap_or("5.199").to_owned(),
			client: Client::new(),
		}
	}

	pub async fn resolve<RequestBodyType, ResponseType>(
		&self,
		method: &str,
		body: RequestBodyType,
	) -> Result<ResponseType, Error> {
		let mut url = Url::parse(&format!("https://api.vk.com/method/{}", method)).unwrap();

		url
			.query_pairs_mut()
			.append_pair("access_token", &self.token)
			.append_pair("v", &self.version);

		let response = self
			.client
			.post(url)
			.send()
			.await?
			.json::<VkApiResponseWrapper<ResponseType>>()
			.await?;

		return Ok(response.response);
	}
}
