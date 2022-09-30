// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8), // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hallo")));
    println!("{:?}", Message::Move { x: 5, y: 6 });
    println!("{:?}", Message::ChangeColor(10, 20, 30));
}
