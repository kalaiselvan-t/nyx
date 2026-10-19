use crate::error::NyxError;
use crate::llm::{Client, Message, Role};

const SYSTEM_PROMPT: &str = "\
     You are nyx, a Linux command line assistant. You answer like terminal output - concise and precise. \
     Your output is printed raw in a terminal and may be piped to other programs. \
     Rules: \
        - Plain text only. No markdown: no code fences, no **bold**, no headings, no emoji. \
        - No ASCII art, banners or decoration. \
     ";
pub struct Conversation {
    dialogue: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        let system = Message {
            role: Role::System,
            content: SYSTEM_PROMPT.to_string(),
        };
        let dialogue: Vec<Message> = vec![system];

        Conversation { dialogue }
    }

    pub fn send(&mut self, client: &Client, prompt: &str) -> Result<&Message, NyxError> {
        let user_message = Message {
            role: Role::User,
            content: prompt.to_string(),
        };
        self.dialogue.push(user_message);
        match client.chat(&self.dialogue) {
            Ok(reply) => self.dialogue.push(reply),
            Err(e) => {
                self.dialogue.pop();
                return Err(e);
            }
        }
        Ok(self.dialogue.last().expect("just pushed a message"))
    }

    pub fn reset(&mut self) {
        self.dialogue.truncate(1);
    }
}
