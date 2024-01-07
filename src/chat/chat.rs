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

#[derive(Debug)]
pub enum MessageStatus {
    Join,
    Leave,
    Unread,
    Read,
    Deleted,
}

#[derive(Debug)]
pub enum MessageKind {
    Direct(String),
    Group(String),
}
#[derive(Debug)]
pub struct Message {
    status: MessageStatus,
    kind: MessageKind,
    contents: Option<String>,
    // time the message was created
    created_at: String,
    // User id of the person who created the message
    created_by: String,
}

impl Message {
    /// Creates a new [`Message`].
    pub fn new(
        kind: MessageStatus,
        contents: Option<String>,
        created_at: String,
        created_by: String,
    ) -> Self {
        Self {
            status: kind,
            contents,
            created_at,
            created_by,
        }
    }
}

struct ChatRoom {
    name: String,
    messages: Vec<Message>,
}
