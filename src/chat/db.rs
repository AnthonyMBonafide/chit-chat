use mysql::{Conn, Pool};
use redis::Commands;

use crate::chat::chat::{Message, MessageKind};

trait Datastore {
    fn get_messages_for_user(&self, id: String) -> Result<Vec<Message>, String>;
    fn mark_message_read(&self, user_id: String, message_id: String) -> Result<(), String>;
    fn send_direct_message(
        &self,
        from_user_id: String,
        to_user_id: String,
        contents: String,
    ) -> Result<(), String>;

    fn send_message_to_chat(
        &self,
        from_user_id: String,
        chat_room_id: String,
        contents: String,
    ) -> Result<(), String>;

    //TODO mark direct and chat room messages deleted
}

// TODO add implmentation for whatever datastore we want to use(maybe Redis or SQL lite, or both
// driven by configuration)
struct MySql {}

impl MySql {
    fn new() -> Self {
        let url = "mysql://root:password@localhost:3307/db_name";
        let pool = Pool::new(url).expect("pool");
        let mut conn = pool.get_conn().expect("connection");

        // Let's create a table for payments.
        conn.query_drop(
            r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )",
        )?;
        Self {}
    }
}
#[cfg(test)]
mod tests {}
