use crate::message::Message;

mod message;

fn main() {
    println!("Hello, world!");

    let mut mess = Message::new(String::from("Hollow World"));
    mess.show();
    
    mess.msg = String::from("YO");
    mess.show();

    mess.msg = String::from("Last Test Message with show()");
    mess.show();

    mess.msg = String::from("Last Test Message with println!()");
    println!("{:?}", mess);

}
