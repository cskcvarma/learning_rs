use std::collections::{HashMap, VecDeque}; // Importing HashMap for key-value storage and VecDeque for efficient double-ended queues
use std::io::{self, Write}; // Importing input/output utilities and write trait for writing to streams
use std::net::{TcpListener, TcpStream}; // Importing TcpListener for server listening and TcpStream for handling connections
use std::sync::mpsc::{self, Receiver, Sender}; // Importing mpsc for message passing
use std::sync::{Arc, Mutex}; // Importing Arc for shared ownership and Mutex for thread-safe access to shared data
use std::thread;

enum RoomMessage {
    SendMessage {
        user_id: usize,
        content: String,
    },
    AddReaction {
        user_id: usize,
        message_index: usize,
        reaction: String,
    },
    ShowRecentMessages {
        sender: Sender<Vec<String>>,
    },
}

enum ManagerMessage {
    EnterRoom {
        user_id: usize,
        room_name: String,
    },
    CreateRoom {
        room_name: String,
    },
    RouteToRoom {
        room_name: String,
        message: RoomMessage,
    },
}

struct ChatRoom {
    messages: VecDeque<(usize, String)>,
    reactions: HashMap<usize, Vec<(usize, String)>>,
    reciever: Receiver<RoomMessage>,
}

impl ChatRoom {
    fn new(reciever: Receiver<RoomMessage>) -> Self {
        Self {
            messages: VecDeque::new(),
            reactions: HashMap::new(),
            reciever,
        }
    }

    fn run(&mut self) {
        while let Ok(message) = self.reciever.recv() {
            match message {
                RoomMessage::SendMessage { user_id, content } => {
                    if self.messages.len() == 10 {
                        self.messages.pop_front();
                    }
                    self.messages.push_back((user_id, content));
                }
                RoomMessage::AddReaction {
                    user_id,
                    message_index,
                    reaction,
                } => {
                    self.reactions
                        .entry(message_index)
                        .or_insert_with(Vec::new)
                        .push((user_id, reaction));
                }
                RoomMessage::ShowRecentMessages { sender } => {
                    let messages = self
                        .messages
                        .iter()
                        .enumerate()
                        .map(|(i, (uid, msg))| format!("{}: [User {}] {}", i + 1, uid, msg))
                        .collect();
                    sender.send(messages).unwrap();
                }
            }
        }
    }
}


// Actor for managing rooms
struct ChatRoomManager {
    rooms: HashMap<String, Sender<RoomMessage>>, // Room name to room sender
    receiver: Receiver<ManagerMessage>,
}

impl ChatRoomManager {
    fn new(receiver: Receiver<ManagerMessage>) -> Self {
        Self {
            rooms: HashMap::new(),
            receiver,
        }
    }

    fn run(&mut self) {
        while let Ok(message) = self.receiver.recv() {
            match message {
                ManagerMessage::EnterRoom { user_id, room_name } => {
                    println!("User {} entered room '{}'.", user_id, room_name);
                }
                ManagerMessage::CreateRoom { room_name } => {
                    if !self.rooms.contains_key(&room_name) {
                        let (tx, rx) = mpsc::channel();
                        let mut room = ChatRoom::new(rx);
                        thread::spawn(move || room.run());
                        self.rooms.insert(room_name, tx);
                    }
                }
                ManagerMessage::RouteToRoom { room_name, message } => {
                    if let Some(room_sender) = self.rooms.get(&room_name) {
                        room_sender.send(message).unwrap();
                    }
                }
            }
        }
    }
}


fn handle_client(stream: TcpStream, sender: Sender<ManagerMessage>, user_id: usize) {
    use std::io::{BufRead, BufReader};
    let mut client = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    writeln!(client, "Welcome User {}!", user_id).unwrap();

    loop {
        writeln!(client, "Options: \n1. Enter Room \n2. Create Room \n3. Send Message \n4. Show Recent Messages \n5. Exit").unwrap();
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                writeln!(client, "Enter room name:").unwrap();
                input.clear();
                reader.read_line(&mut input).unwrap();
                let room_name = input.trim().to_string();
                sender
                    .send(ManagerMessage::EnterRoom { user_id, room_name })
                    .unwrap();
            }
            "2" => {
                writeln!(client, "Enter new room name:").unwrap();
                input.clear();
                reader.read_line(&mut input).unwrap();
                let room_name = input.trim().to_string();
                sender
                    .send(ManagerMessage::CreateRoom { room_name })
                    .unwrap();
            }
            "3" => {
                writeln!(client, "Enter room name:").unwrap();
                input.clear();
                reader.read_line(&mut input).unwrap();
                let room_name = input.trim().to_string();
                writeln!(client, "Enter your message:").unwrap();
                input.clear();
                reader.read_line(&mut input).unwrap();
                let content = input.trim().to_string();
                sender
                    .send(ManagerMessage::RouteToRoom {
                        room_name,
                        message: RoomMessage::SendMessage { user_id, content },
                    })
                    .unwrap();
            }
            "4" => {
                writeln!(client, "Enter room name:").unwrap();
                input.clear();
                reader.read_line(&mut input).unwrap();
                let room_name = input.trim().to_string();
                let (tx, rx) = mpsc::channel();
                sender
                    .send(ManagerMessage::RouteToRoom {
                        room_name,
                        message: RoomMessage::ShowRecentMessages { sender: tx },
                    })
                    .unwrap();
                if let Ok(messages) = rx.recv() {
                    writeln!(client, "Recent Messages:").unwrap();
                    for msg in messages {
                        writeln!(client, "{}", msg).unwrap();
                    }
                }
            }
            "5" => {
                writeln!(client, "Goodbye!").unwrap();
                break;
            }
            _ => {
                writeln!(client, "Invalid Option.").unwrap();
            }
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut manager = ChatRoomManager::new(rx);
    thread::spawn(move || manager.run());

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut user_id_counter = 0;

    println!("Server running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
            let sender = tx.clone();
            user_id_counter += 1;
            let user_id = user_id_counter;
            thread::spawn(move || handle_client(stream, sender, user_id));
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
        }
        }
    }
}
