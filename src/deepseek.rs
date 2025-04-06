use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deepseek {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeepseekAPIOutput {
    pub choices: Vec<Choice>,
}
pub async fn prompt_deepseek(messages: Vec<Message>) -> Result<DeepseekAPIOutput, Box<dyn Error>> {
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

pub async fn process_stream(messages: Vec<Message>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let api_key =
        env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY environment variable not set");

    let deepseek = Deepseek {
        model: "deepseek-chat".to_string(),
        messages,
        stream: true,
    };

    let mut res = client
        .post("https://api.deepseek.com/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&deepseek)
        .send()
        .await?;

    let mut stdout_handle = std::io::stdout();

    while let Some(chunk) = res.chunk().await? {
        stdout_handle.write_all(&chunk)?;
        stdout_handle.flush()?;
    }

    Ok(())
}
