use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Deepseek {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    index: i32,
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeepseekAPIOutput {
    choices: Vec<Choice>,
}

async fn prompt_deepseek(messages: Vec<Message>) -> Result<DeepseekAPIOutput, Box<dyn Error>> {
    let client = Client::new();

    let api_key =
        env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY environment variable not set");

    let deepseek = Deepseek {
        model: "deepseek-chat".to_string(),
        messages,
        stream: false,
    };

    let res = client
        .post("https://api.deepseek.com/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&deepseek)
        .send()
        .await?;
    let output: DeepseekAPIOutput = res.json().await?;
    Ok(output)
}

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
