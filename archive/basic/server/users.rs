use crate::server::message::Message;

use super::message::ChatMessage;

pub struct User {
    pub username: String,
}

impl User {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_owned(),
        }
    }

    pub fn send_message(&self, content: &str) -> ChatMessage {
        println!("{} is sending a message: '{}'", self.username, content);
        ChatMessage::new(content)
    }

    pub fn react_to_message(&self, message: &mut ChatMessage, reaction: &str) {
        println!(
            "{} is reacting to message: '{}'",
            self.username,
            message.get_content()
        );
        message.react(&self.username, reaction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("Alice");
        assert_eq!(user.username, "Alice");
    }

    #[test]
    fn test_user_send_a_message() {
        let user = User::new("Bob");
        let message = user.send_message("Hello, world!");

        assert_eq!(message.get_content(), "Hello, world!");
    }

    #[test]
    fn test_user_react_to_a_message() {
        let alice = User::new("Alice");
        let bob = User::new("Bob");
        let mut message = alice.send_message("Hello, world!");

        bob.react_to_message(&mut message, "Wassup!");

        assert!(message.has_reactions());
        assert_eq!(message.reactions.get("Bob"), Some(&"Wassup!".to_string()));
    }
}
