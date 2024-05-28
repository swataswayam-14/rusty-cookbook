enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move message: x={}, y={}", x, y),
            Message::Write(s) => println!("Write message: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color message: R={}, G={}, B={}", r, g, b),
        }
    }
}

//The body of the method would use self to get the value that we called the method on. In this example, we’ve created a variable m that has the value Message::Write(String::from("hello")), and that is what self will be in the body of the call method when m.call() runs.

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}