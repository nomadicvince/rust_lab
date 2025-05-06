# OpenAI Rust Chatbot CLI

A simple command-line chatbot using Rust and the OpenAI API, implemented with a Message Consumer Provider (MCP) architecture.

## Features

- Clean separation of concerns using the MCP pattern
- Asynchronous communication with the OpenAI API
- Conversation history tracking
- Simple command-line interface

## Prerequisites

- Rust and Cargo installed
- An OpenAI API key

## Setup

1. Clone this repository
2. Copy the `.env.example` file to `.env` and add your OpenAI API key:
   ```
   OPENAI_API_KEY=your_openai_api_key_here
   ```
3. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the chatbot with:

```
cargo run --release
```

Type your messages and press Enter. The AI will respond based on the conversation history.
Type `exit` to quit the application.

## MCP Architecture

This project implements a simple Message Consumer Provider pattern:

- **Message**: The core data structure representing communication
- **Provider**: Processes messages and generates responses (OpenAI API)
- **Consumer**: Handles displaying or otherwise consuming messages (Console)

This architecture makes it easy to extend the application with different providers (e.g., local models) or consumers (e.g., GUI, web interface).

## Project Structure

```
openai_chatbot/
├── .env                  # Environment variables (API key)
├── Cargo.toml            # Project dependencies and metadata
├── README.md             # Project documentation
└── src/                  # Source code directory
    ├── main.rs           # Application entry point
    └── mcp/              # MCP architecture components
        ├── mod.rs        # Module declaration file
        ├── message.rs    # Message data structure
        ├── provider.rs   # Provider trait definition
        ├── consumer.rs   # Consumer trait definition
        ├── openai.rs     # OpenAI implementation of Provider
        └── console.rs    # Console implementation of Consumer
```

### Key Components Explained

1. **Root Directory**
   - `.env`: Stores environment variables, specifically your OpenAI API key
   - `Cargo.toml`: Defines project metadata and dependencies
   - `README.md`: Documentation for the project

2. **Source Code (`src/`)**
   - `main.rs`: The entry point that ties everything together
     - Sets up the provider and consumer
     - Implements the chat loop
     - Handles user input and output

3. **MCP Module (`src/mcp/`)**
   - `mod.rs`: Declares all submodules within the MCP module
   - `message.rs`: Defines the `Message` struct, the core data structure for communication
   - `provider.rs`: Contains the `Provider` trait that defines how messages are processed
   - `consumer.rs`: Contains the `Consumer` trait that defines how to handle responses
   - `openai.rs`: Implements the `Provider` trait using the OpenAI API
   - `console.rs`: Implements the `Consumer` trait for console output

### Architecture Flow

The structure directly reflects the MCP pattern:

1. **Message**: Defined in `message.rs`, this is the data structure that flows through the system
2. **Provider**: Interface in `provider.rs` with implementation in `openai.rs`
   - Takes messages, processes them, and returns responses
3. **Consumer**: Interface in `consumer.rs` with implementation in `console.rs`
   - Takes messages and displays them to the user

This modular architecture makes it easy to:
- Replace the OpenAI provider with another AI provider
- Add a new consumer (like a GUI or web interface) without changing the rest of the code
- Extend the message structure if needed

The separation of concerns makes the code more maintainable and testable, as each component has a single responsibility and can be developed or modified independently.

## Customization

You can modify the model used in `src/mcp/openai.rs` by changing the model name in the `OpenAIRequest` struct.

## License

MIT
