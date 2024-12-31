pub trait Message {
    fn get_content(&self) -> &str;
    fn react(&mut self, username: &str, reaction: &str);
    fn show_reactions(&self);
}

pub struct ChatMessage {
    content: String,
    pub reactions: std::collections::HashMap<String, String>,
}

impl ChatMessage {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            reactions: std::collections::HashMap::new(),
        }
    }

    pub fn has_reactions(&self) -> bool {
        !self.reactions.is_empty()
    }
}

impl Message for ChatMessage {
    fn get_content(&self) -> &str {
        &self.content
    }

    fn react(&mut self, username: &str, reaction: &str) {
        self.reactions.insert(username.to_owned(), reaction.to_owned());
    }

    fn show_reactions(&self) {
        println!("Reactions for message: '{}'", self.content);
        for (user, reaction) in &self.reactions {
            println!(" {} reacted with {}", user, reaction);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = ChatMessage::new("Hello, world!");
        assert_eq!(message.get_content(), "Hello, world!");
    }

    #[test]
    fn test_message_react() {
        let mut message = ChatMessage::new("Test message");
        message.react("Alice", "👍");
        message.react("Bob", "😂");

        assert_eq!(message.reactions.len(), 2);
        assert_eq!(message.reactions.get("Alice"), Some(&"👍".to_string()));
        assert_eq!(message.reactions.get("Bob"), Some(&"😂".to_string()));

    }

    #[test]
    fn test_message_overwrite_reaction() {
        let mut message = ChatMessage::new("Test message");
        message.react("Alice", "👍");
        message.react("Alice", "😂");

        assert_eq!(message.reactions.len(), 1);
        assert_eq!(message.reactions.get("Alice"), Some(&"😂".to_string()))
    }

    #[test]
    fn test_message_show_reaction() {
        let mut message = ChatMessage::new("Test message");
        message.react("Alice", "👍");
        message.react("Bob", "😂");

        use std::io::Write;
        let mut buffer = std::io::Cursor::new(Vec::new());

        {
            let writer = &mut buffer;
            writeln!(writer, "Reactions for message: '{}'", message.get_content()).unwrap();
            for (user, reaction) in &message.reactions {
                writeln!(writer, "  {} reacted with: {}", user, reaction).unwrap();
            }

        }

        let output = String::from_utf8(buffer.into_inner()).expect("Invalid UTF-8");

        assert!(output.contains("Alice reacted with: 👍"));
        assert!(output.contains("Bob reacted with: 😂"))
    }
}