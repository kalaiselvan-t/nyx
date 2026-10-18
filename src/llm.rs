use crate::config::Config;
use crate::error::NyxError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

pub struct Client {
    agent: ureq::Agent,
    api_key: String,
    model: String,
}

impl Client {
    pub fn new(config: &Config) -> Client {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .ip_family(ureq::config::IpFamily::Ipv4Only)
            .build()
            .into();
        Client {
            agent,
            api_key: config.api_key.clone(),
            model: config.model.clone(),
        }
    }

    pub fn chat(&self, messages: &[Message]) -> Result<Message, NyxError> {
        let url: &str = "https://openrouter.ai/api/v1/chat/completions";
        let authorization: String = format!("Bearer {}", self.api_key);
        let request = ChatRequest {
            model: &self.model,
            messages,
        };
        let response: ChatResponse = self
            .agent
            .post(url)
            .header("Authorization", authorization)
            .header("content-type", "application/json")
            .send_json(&request)?
            .body_mut()
            .read_json()?;

        response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message)
            .ok_or_else(|| NyxError::BadResponse("Response had no choices".to_string()))
    }
}

pub fn call(client: &Client, prompt: &str) -> Result<Message, NyxError> {
    const SYSTEM_PROMPT: &str = "You are nyx, a Linux command line assistant. You answer like terminal output - concise and precise. \
     Your output is printed raw in a terminal and may be piped to other programs. \
     Rules: \
        - Plain text only. No markdown: no code fences, no **bold**, no headings, no emoji. \
        - No ASCII art, banners or decoration. \
     ";

    let system_message = Message {
        role: Role::System,
        content: SYSTEM_PROMPT.to_string(),
    };

    let user_message = Message {
        role: Role::User,
        content: prompt.to_string(),
    };

    client.chat(&[system_message, user_message])
}
