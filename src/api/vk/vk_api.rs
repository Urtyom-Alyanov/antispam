use reqwest::{Client, Error, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
	) -> Result<ResponseType, Error> where
	    RequestBodyType: Serialize,
		ResponseType: for<'de> Deserialize<'de>,
	{
		let url = Url::parse(&format!("https://api.vk.com/method/{}", method)).unwrap();

		let mut full_body = json!(body);

		if let Some(obj) = full_body.as_object_mut() {
            obj.insert("access_token".to_string(), json!(self.token));
            obj.insert("v".to_string(), json!(self.version));
        }

		let response = self
			.client
			.post(url)
			.json(&full_body)
			.send()
			.await?
			.json::<VkApiResponseWrapper<ResponseType>>()
			.await?;

		return Ok(response.response);
	}
}
