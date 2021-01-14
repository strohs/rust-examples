
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// NOTE Each struct is allowed to have multiple impl blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn marea(&mut self) -> u32 {
        self.width += self.width + 2;
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    // associated function that builds a square Rectangle
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// examples of tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);



fn main() {

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    // the :: syntax is used for both associated functions and namespaces created by modules
    let square1 = Rectangle::square(25);


    println!("rect1 is {:?}", rect1);
    println!("square1 is {:?}", square1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // tuple struct creation, these are new Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
