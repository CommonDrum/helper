use std::env;
use std::error::Error;

mod deepseek;
use deepseek::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
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

    let output = prompt_deepseek(messages).await?;
    println!("{:?}", output.choices[0].message.content);

    Ok(())
}
