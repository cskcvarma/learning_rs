use server::{message::Message, users::User};

mod server;


fn main() {
    let user1 = User::new("Alice");
    let user2 = User::new("Bob");

    // User 1 sends a message.
    let mut message = user1.send_message("Hello, everyone!");

    // User 1 reacts to their own message.
    user1.react_to_message(&mut message, "😊");

    // User 2 reacts to User 1's message.
    user2.react_to_message(&mut message, "👍");

    // Display the message content and reactions.
    println!("Message: {}", message.get_content());
    message.show_reactions();
}
