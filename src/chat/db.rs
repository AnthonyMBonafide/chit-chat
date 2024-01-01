use crate::chat::chat::Message;

trait Datastore {
    fn get_messages_for_user(id: String) -> Result<Vec<Message>, String>;
    fn mark_message_read(user_id: String, message_id: String) -> Result<(), String>;
    fn send_direct_message(
        from_user_id: String,
        to_user_id: String,
        contents: String,
    ) -> Result<(), String>;

    fn send_message_to_chat(
        from_user_id: String,
        chat_room_id: String,
        contents: String,
    ) -> Result<(), String>;

    //TODO mark direct and chat room messages deleted
}

// TODO add implmentation for whatever datastore we want to use(maybe Redis or SQL lite, or both
// driven by configuration)
