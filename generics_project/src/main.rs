#[derive(Debug)]
enum DigitalContent{
    AudioFile,
    Videofile
}
#[derive(Debug)]
struct ChatMessage<T>{
    content: T,
    time: String
}
impl ChatMessage<DigitalContent>{
    fn consume_entertainment(&self){
        println!("{:?}",self.content);
    }
}
impl<T> ChatMessage<T>{
    fn retrieve_time(&self) -> String{
        self.time.clone()
    }
}

fn main() {

    let message =ChatMessage{
        content:DigitalContent::AudioFile,
        time: String::from("xyz")
    };

    message.consume_entertainment();
    println!("{}",message.retrieve_time());

    println!("{:#?}",message);

    let message =ChatMessage{
        content:"abcd",
        time: String::from("20:20:55")
    };
    println!("{}",message.retrieve_time());

    println!("{:#?}",message);

    let message =ChatMessage{
        content:"abcd".to_string(),
        time: String::from("xyz")
    };
    println!("{}",message.retrieve_time());
    println!("{:#?}",message);
}
