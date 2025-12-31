// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },           // 命名字段方式（像结构体）
    Echo(String),                       // 带一个 String
    ChangeColor(i32, i32, i32),         // tuple 方式，三个 i32（RGB）
    Quit,                               // unit-like，不带数据
}


impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
