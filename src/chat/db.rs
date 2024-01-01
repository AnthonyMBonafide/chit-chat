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

struct Redis {
    client: redis::Client,
    connection: redis::Connection,
}

impl Redis {
    fn new(redis_url: String) -> Self {
        // connect to redis
        let client = redis::Client::open(redis_url).expect("a valid client");
        let connection = client.get_connection().expect("a valid connection");
        Redis { client, connection }
    }
}

impl Datastore for Redis {
    fn get_messages_for_user(&self, id: String) -> Result<Vec<Message>, String> {
        let mut conn = self.client.get_connection().expect("valid connection");
        let _res: Vec<String> = conn.hkeys("anthony:message").expect("response");
        println!("Test {:?}", _res);
        let v: Vec<Message> = vec![Message::new(
            MessageKind::Unread,
            Some("".to_string()),
            "".to_string(),
            "".to_string(),
        )];
        Ok(v)
    }

    fn mark_message_read(&self, user_id: String, message_id: String) -> Result<(), String> {
        todo!()
    }

    fn send_message_to_chat(
        &self,
        from_user_id: String,
        chat_room_id: String,
        contents: String,
    ) -> Result<(), String> {
        todo!()
    }

    fn send_direct_message(
        &self,
        from_user_id: String,
        to_user_id: String,
        contents: String,
    ) -> Result<(), String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use redis::Commands;

    use super::{Datastore, Redis};

    #[test]
    fn redis_get_messages() {
        let redis_url = "redis://127.0.0.1:6379".to_string();
        // Setup
        let client = redis::Client::open(redis_url.clone()).expect("a valid client");
        let mut connection = client.get_connection().expect("a valid connection");
        let _: () = connection
            .hset(
                "anthony:message".to_string(),
                "message-2".to_string(),
                "hello",
            )
            .expect("value to be set");

        // Test
        let client: Redis = Redis::new(redis_url);
        let messages = client
            .get_messages_for_user("anthony".to_string())
            .expect("valid results");
        println!("{:?}", messages);

        // Cleanup
        let _: () = connection
            .hdel("anthony:message".to_string(), "message-2".to_string())
            .expect("result");
    }
}
