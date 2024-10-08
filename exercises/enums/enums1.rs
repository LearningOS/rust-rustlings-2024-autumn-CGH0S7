// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit(u8),
    Echo(String),
    Move(String, String),
    ChangeColor(i8),
}

fn main() {
    println!("{:?}", Message::Quit(1));
    println!("{:?}", Message::Echo(String::from("RUSTLINGS")));
    println!(
        "{:?}",
        Message::Move(String::from("Up"), String::from("Down"))
    );
    println!("{:?}", Message::ChangeColor(-128));
}
