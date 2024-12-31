use super::users::User;
use super::message::{ChatMessage, Message};
use std::sync::{Arc, Mutex};

pub struct Room {
    name: String,
    users: Vec<Arc<Mutex<User>>>, // List of users in the room, wrapped in Arc<Mutex> for shared, thread safe access.
    messages: Vec<Arc<Mutex<ChatMessage>>>, // List of messages in the room, wrapped similarly.
}

impl Room {

    //&str is typically used for reading string data.
    // String is used when you need ownership or the ability to modify the string.

    pub fn new(room_name: &str) -> Self {
        Self {
            name: room_name.to_owned(),
            users: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User){
        self.users.push(Arc::new(Mutex::new(user)));
        println!("User joined room: {}", self.name);
    }

    pub fn add_message(&mut self, content: &str) {
        let message = ChatMessage::new(content);
        self.messages.push(Arc::new(Mutex::new(message)));
        println!("Message added to room '{}': {}", self.name, content);
    }

    pub fn react_to_last_message(&self, username: &str, reaction: &str) {
        if let Some(last_message) = self.messages.last() {
            let mut message = last_message.lock().unwrap();
            message.react(username, reaction);
            println!("{} reacted to the last message in '{}'", username, self.name);
        } else {
            println!("No messages to react to in '{}'", self.name);
        }
    }

    pub fn show_all_messages(&self) {
        println!("Messaged in room '{}':", self.name);
        for message in &self.messages {
            let locked_message = message.lock().unwrap();
            println!("{}", locked_message.get_content());
            locked_message.show_reactions();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOM_NAME: &str = "Test Room";
    const ALICE: &str = "Alice";
    const BOB: &str = "Bob";
    const CONTENT: &str = "Content";
    const REACTION: &str = "Wassup!";

    fn create_test_room() -> Room {
        Room::new(ROOM_NAME)
    }

    fn create_test_user(name: &str) -> User {
        User::new(name)
    }  

    #[test]
    fn test_room_creation() {
        let room = create_test_room();
        assert_eq!(room.name, ROOM_NAME);
        assert!(room.users.is_empty());
        assert_eq!(room.messages.len(), 0);
    }

    #[test]
    fn add_user_to_room() {
        let mut room =  create_test_room();

        room.add_user(create_test_user(ALICE));
        
        assert_eq!(room.users.len(), 1)
    }

    #[test]
    fn add_message_to_room() {
        let mut room = create_test_room();
        room.add_user(create_test_user(ALICE));

        room.add_message(CONTENT);
        
        let last_message = room.messages.last().expect("No message found in the room");
        let last_message_content = last_message.lock().unwrap();
        assert_eq!(last_message_content.get_content(), CONTENT);
    }

    #[test]
    fn react_to_last_message() {
        let mut room: Room = create_test_room();
        let alice = create_test_user(ALICE);
        let bob = create_test_user(BOB);
        room.add_user(alice);
        room.add_user(bob);

        room.add_message(CONTENT);
        room.react_to_last_message(BOB, REACTION);
        
        let last_message = room.messages.last().expect("No messages found in the room");
        
        assert!(last_message.lock().unwrap().has_reactions());
        assert_eq!(last_message.lock().unwrap().reactions.get(BOB), Some(&REACTION.to_string()));
    }
}