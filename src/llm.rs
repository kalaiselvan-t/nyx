use serde_json::json;
use crate::config::Config;
use crate::error::NyxError;

pub fn call(prompt: &str, config: &Config) -> Result<String, NyxError> {
    const SYSTEM_PROMPT: &str = "You are nyx, a Linux command line assistant. You answer like terminal output - concise and precise. \
     Your output is printed raw in a terminal and may be piped to other programs. \
     Rules: \
        - Plain text only. No markdown: no code fences, no **bold**, no headings, no emoji. \
        - No ASCII art, banners or decoration. \
     ";

    let url: &str = "https://openrouter.ai/api/v1/chat/completions";
    let authorization: String = format!("Bearer {}", config.api_key);

    let data = json!({
        "model": config.model,
        "messages": [
            {
                "role": "system",
                "content": SYSTEM_PROMPT,
            },
            {
                "role": "user",
                "content": prompt,
            }
        ],
    });

    let ureq_agent: ureq::Agent = ureq::Agent::config_builder()
        .ip_family(ureq::config::IpFamily::Ipv4Only)
        .build()
        .into();

    let request = ureq_agent
        .post(url)
        .header("Authorization", authorization)
        .header("content-type", "application/json");

    let mut response = request.send_json(&data)?;
    let body: serde_json::Value = response.body_mut().read_json()?;

    match body["choices"][0]["message"]["content"].as_str() {
        Some(text) => Ok(text.to_string()),
        None => Err(NyxError::BadResponse(body.to_string())),
    }
}