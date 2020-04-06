

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    width: usize
}
impl Draw for Button {
    fn draw(&self) {
        println!("drawing button {}", self.width);
    }
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }

    pub fn new() -> Screen<T> {
        Screen {
            components: Vec::new()
        }
    }
    pub fn push(&mut self, drawable: T) {
        self.components.push(drawable);
    }
}

fn main() {
    let mut screen: Screen<Button> = Screen::new();
    let b = Button { width: 455 };
    screen.push(b);
    screen.run();
    println!("done");
}