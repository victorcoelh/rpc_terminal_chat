pub struct UserChat{
    pub messages: Vec<String>   
}

impl UserChat {
    pub fn new() -> UserChat {
        UserChat { messages: Vec::new() }
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
    }
}
