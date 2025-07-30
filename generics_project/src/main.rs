#[derive(Debug)]
enum DigitalContent{
    AudioFile,
    Videofile
}
struct ChatMessage<T>{
    content: T,
    time: String
}

fn main() {
    println!("Hello, world!");
}
