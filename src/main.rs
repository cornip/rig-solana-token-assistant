use dotenv::dotenv;
use rig::{providers, completion::Prompt};
use std::env;
use tools::swap::SwapTool;
use tools::transfer::TransferTool;
use std::io::Write;
mod solana;
mod types;
mod tools;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let openai_client = providers::openai::Client::new(&openai_api_key);
    let swap_tool = SwapTool::new();
    let transfer_tool = TransferTool::new();
    let agent = openai_client
        .agent("gpt-4")
        .preamble("I am a Solana token assistant that can analyze top holders, execute token swaps using Jupiter, and transfer SOL or SPL tokens to other addresses. I can help you analyze token distributions, perform token swaps, or send tokens. How can I assist you today?")
        .tool(swap_tool)
        .tool(transfer_tool)
        .build();

    println!("Solana Token Assistant Ready! (Type 'exit' to quit)");
    println!("Examples:");
    println!("1. Swap 1 SOL to USDC");
    println!("2. Transfer 0.5 SOL to address ABC...");
    println!("3. Transfer 10 USDC to address ABC...");
    
    // let input = "swap 0.01 sol to rina (6wUfdjiBtXjiWTfwGabBqybVTCAFoS9iD3X6t9v1pump) with slippage 1%";
    // let response = agent.prompt(input).await?;
    // println!("Response: {}", response);

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "exit" {
            break;
        }

        match agent.prompt(input).await {
            Ok(response) => println!("Response: {}", response),
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    }

    Ok(())
}