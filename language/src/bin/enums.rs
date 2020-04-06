#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// ex. of defining a method on enum
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("i am quit"),
            Message::Move {x,y}=> println!("i am move {},{}", x,y),
            Message::Write(s) => println!("i am write: {}", s),
            Message::ChangeColor(r,g,b) => println!("I am ChangeColor {} {} {}",r,g,b)
        }
    }
}



fn main() {
    let m = Message::Write(String::from("THE WRITE MESSAGE"));
    let m1 = Message::Move {x:99, y:88};
    m.call();
    m1.call();

    let mut count = 0;
    if let Message::Move {x,y} = m1 {
        println!("move message to {} {}", x,y);
    } else {
        count += 1;
    }
    println!("count={}", count);
}