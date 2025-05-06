use async_trait::async_trait;
use crate::mcp::message::Message;

#[async_trait]
pub trait Provider {
    async fn process_message(&self, messages: &[Message]) -> Result<Message, Box<dyn std::error::Error>>;
}
