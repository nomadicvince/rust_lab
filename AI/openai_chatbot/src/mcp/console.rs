use crate::mcp::consumer::Consumer;
use crate::mcp::message::Message;

pub struct ConsoleConsumer;

impl ConsoleConsumer {
    pub fn new() -> Self {
        ConsoleConsumer
    }
}

impl Consumer for ConsoleConsumer {
    fn consume_message(&self, message: &Message) {
        println!("\n{}", message.content);
    }
}
