use lib::conversation::Manager;

use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, json};
use std::{env, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,

    #[arg(short = 'm', long, env = "USE_LLM", default_value = "anthropic/claude-haiku-4.5")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // println!("Using LLM '{}'", &args.model);

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);
    let mut manager = Manager::new(&args.prompt);
    // eprintln!("Logs from your program will appear here"); // for debugging
    loop {
        // 1. Send current history to the model
        let response: Value = client
            .chat()
            .create_byot(json!({
                "model": &args.model,
                "messages": manager.messages,
                "tools": manager.tools,
            }))
            .await?;

        let message = &response["choices"][0]["message"];
        manager.add_message(message.clone());

        if let Some(tool_calls) = message["tool_calls"].as_array() {
            for call in tool_calls {
                manager.handle_tool_call(call)?;
            }
            continue; // Continue the loop to send tool results back to the LLM
        }

        // 3. If no tool calls, the AI has provided its final answer
        if let Some(content) = message["content"].as_str() {
            println!("{}", content);
        }
        break;
    }

    Ok(())
}
