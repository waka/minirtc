extern crate ws;

mod messages;

pub struct Server {
    pub ws: ws::Sender
}

impl ws::Handler for Server {
    fn on_open(&mut self, handshake: ws::Handshake) -> ws::Result<()> {
        if let Some(ip_addr) = handshake.remote_addr()? {
            println!("WebSocket connection opened from {}", ip_addr)
        } else {
            println!("WebSocket failed to open connection.")
        }
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Message: {}", &msg);
        if let Ok(text) = msg.into_text() {
            let reply = messages::make_reply(text);
            println!("Reply: {}", &reply);
            let _= self.ws.broadcast(reply);
        }
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
        println!("Shutting down server after first connection closes.");
        self.ws.shutdown().unwrap();
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("WebSocket error for ({:?})", err);
    }
}
