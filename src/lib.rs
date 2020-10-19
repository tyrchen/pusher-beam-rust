use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE, AUTHORIZATION};
use serde::{ Serialize};
use serde_json::{Value, json};

const SDK_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct PusherBeam {
    instance_id: String,
    secret_key: String,
}

#[derive(Debug, Serialize)]
pub struct Payload {
    interests: Vec<String>,
    #[serde(skip_serializing_if = "Value::is_null")]
    web: Value,
    #[serde(skip_serializing_if = "Value::is_null")]
    fcm: Value,
    #[serde(skip_serializing_if = "Value::is_null")]
    apns: Value,
}

impl PusherBeam {
    pub fn new(instance_id: &str, secret_key: &str) -> Self {
        Self {
            instance_id: instance_id.to_owned(),
            secret_key: secret_key.to_owned(),
        }
    }

    pub async fn publish(&self, payload: &Payload) -> Result<()>  {

        println!("payload: {:?}", json!(payload));

        let client = self.build_client();
        let resp =  client.json(&json!(payload)).send().await?;
        if resp.status() == 200 {
            Ok(())
        } else {
            Err(anyhow!("Failed to publish the request: {}. Error: {}", serde_json::to_string(&payload).unwrap(), resp.text().await?))
        }

    }

    // private function
    fn build_client(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        let url = format!("https://{}.pushnotifications.pusher.com/publish_api/v1/instances/{}/publishes", self.instance_id, self.instance_id);
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, format!("Bearer {}", self.secret_key).parse().unwrap());
        headers.insert("X-Pusher-Library", format!("pusher-push-notifications-node {}", SDK_VERSION).parse().unwrap());

        client.post(&url).headers(headers)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    #[tokio::test]
    async fn it_works() -> Result<()> {
        // To run this test, you need to provide pusher settings in envar with below names.
        let instance_id = env::var("PUSHER_BEAM_INSTANCE_ID").unwrap();
        let secret = env::var("PUSHER_BEAM_SECRET").unwrap();
        let pusher = PusherBeam::new(&instance_id, &secret);
        let request = r#"
        {"web":{"notification":{"title":"Hello","body":"Hello, world!"}}}
        "#;
        let payload = Payload {
            interests: vec!["hello".to_owned(), "hi".to_owned()],
            web: serde_json::from_str(request)?,
            fcm: Value::Null,
            apns: Value::Null,
        };
        pusher.publish(&payload).await.unwrap();
        Ok(())
    }
}
