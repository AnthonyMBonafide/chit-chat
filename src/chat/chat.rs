struct User {
    name: String,
    chat_rooms: Vec<Chat>,
    direct_messages: Vec<Message>,
    // used to display time in the correct zone
    local: String,
}

enum MessageKind {
    Join,
    Leave,
    Unread,
    Read,
}

struct Message {
    kind: MessageKind,
    contents: String,
    // time the message was created
    created_at: String,
    // User id of the person who created the message
    created_by: String,
}

struct Chat {
    name: String,
    messages: Vec<Message>,
}
