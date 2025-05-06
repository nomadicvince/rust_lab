use crate::mcp::message::Message;

pub trait Consumer {
    fn consume_message(&self, message: &Message);
}
