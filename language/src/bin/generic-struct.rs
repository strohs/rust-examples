struct ThingHolder<T> {
    thing: T
}

impl ThingHolder<u32> {
    fn print_thing(&self) {
        println!("I'm specialized! {}", self.thing);
    }
}

impl ThingHolder<i32> {
    fn print_thing(&self) {
        println!("I'm also specialized! {}", self.thing);
    }
}

fn main() {
    let th = ThingHolder { thing: 5u32 };
    th.print_thing();

    let th = ThingHolder { thing: 5i32 };
    th.print_thing();

    //let th = ThingHolder { thing: "compile error" };
    //th.print_thing();
}