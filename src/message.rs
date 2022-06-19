#[derive(Debug)]
pub struct Message{
    pub msg: String,
}

impl Message{  
    pub fn new (msg: String) -> Message {
        Message { msg }
    }
    pub fn show(&self) {
        println!("{}", self.msg);
    }
}
