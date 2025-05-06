use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use crate::mcp::message::Message;
use crate::mcp::provider::Provider;

pub struct OpenAIProvider {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<MessageFormat>,
}

#[derive(Serialize)]
struct MessageFormat {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    role: String,
    content: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();
        OpenAIProvider { api_key, client }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .expect("Failed to create Authorization header"),
        );
        headers
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn process_message(&self, messages: &[Message]) -> Result<Message, Box<dyn std::error::Error>> {
        // Convert messages to the format expected by OpenAI
        let openai_messages: Vec<MessageFormat> = messages
            .iter()
            .map(|msg| MessageFormat {
                role: msg.role.clone(),
                content: msg.content.clone(),
            })
            .collect();

        // Create request payload
        let request = OpenAIRequest {
            model: "gpt-4o".to_string(), // Using GPT-4o, adjust as needed
            messages: openai_messages,
        };

        // Send request to OpenAI API
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(self.build_headers())
            .json(&request)
            .send()
            .await?;

        // Check if request was successful
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API error: {}", error_text).into());
        }

        // Parse the response
        let openai_response: OpenAIResponse = response.json().await?;

        // Extract the assistant's message
        if let Some(choice) = openai_response.choices.first() {
            let message = Message {
                role: choice.message.role.clone(),
                content: choice.message.content.clone(),
            };
            Ok(message)
        } else {
            Err("No response received from OpenAI".into())
        }
    }
}
