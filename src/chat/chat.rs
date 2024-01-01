// Represents a person with a connection to the system.
// A user can have direct messages or belong to a chatroom
// and has their own version of a message with timestamps
// and statuses
struct User {
    id: String,
    name: String,
    chat_rooms: Vec<ChatRoom>,
    direct_messages: Vec<Message>,
    // used to display time in the correct zone
    timezone: String,
}

enum MessageKind {
    Join,
    Leave,
    Unread,
    Read,
    Deleted,
}

pub struct Message {
    kind: MessageKind,
    contents: String,
    // time the message was created
    created_at: String,
    // User id of the person who created the message
    created_by: String,
}

struct ChatRoom {
    name: String,
    messages: Vec<Message>,
}
