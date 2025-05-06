use dotenv::dotenv;
use std::env;
use std::io::{self, Write};

mod mcp;
use mcp::consumer::Consumer;
use mcp::message::Message;
use mcp::provider::Provider;
use mcp::openai::OpenAIProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get API key from environment variable
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    
    // Create an OpenAI provider
    let provider = OpenAIProvider::new(api_key);
    
    // Create a simple console consumer
    let consumer = mcp::console::ConsoleConsumer::new();
    
    println!("OpenAI Chatbot (type 'exit' to quit)");
    println!("-------------------------------------");
    
    // Start conversation loop
    let mut conversation_history: Vec<Message> = Vec::new();
    
    loop {
        // Get user input
        print!("> ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        
        // Trim the input
        let user_input = user_input.trim();
        
        // Check if user wants to exit
        if user_input.to_lowercase() == "exit" {
            break;
        }
        
        // Create user message
        let user_message = Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        };
        
        // Add user message to conversation history
        conversation_history.push(user_message.clone());
        
        // Process the message through the provider
        match provider.process_message(&conversation_history).await {
            Ok(response) => {
                // Add assistant response to conversation history
                conversation_history.push(response.clone());
                
                // Consume the message (display it)
                consumer.consume_message(&response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    
    println!("Goodbye!");
    
    Ok(())
}
