use std::fmt::Debug;

/// if you want an owned trait object, then a Box is the way to go

// types that implement Draw must also implement Debug
pub trait Draw: Debug {
    fn draw(&self);
}



#[derive(Debug)]
pub struct Button {
    id: usize
}
impl Draw for Button {
    fn draw(&self) {
        println!("drawing button id={}", self.id);
    }
}

#[derive(Debug)]
pub struct DropDown {
    id: usize
}
impl Draw for DropDown {
    fn draw(&self) {
        println!("drawing DropDown id={}", self.id);
    }
}

#[derive(Debug)]
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {

    pub fn new() -> Screen {
        Screen {
            components: Vec::new()
        }
    }

    pub fn push(&mut self, drawable: impl Draw + 'static) {
        self.components.push(Box::new(drawable) );
    }

    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


pub fn main() {
    let b = Button { id: 343 };
    let dd = DropDown { id: 6743 };
    let mut screen = Screen::new();
    screen.push(dd);
    screen.push(b);
    screen.run();
    dbg!(screen);
}