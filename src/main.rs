use std::env;
use std::error::Error;

mod deepseek;
use deepseek::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let query = args.join(" ");

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a QuickHelper, your task is to anwser user queries in very short manner. The anwsers still need to be helpful".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: query,
        },
    ];

    process_stream(messages).await?;

    Ok(())
}
