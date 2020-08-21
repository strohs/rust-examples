use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

/// another variation of the List definition from:
/// [https://doc.rust-lang.org/book/ch15-01-box.html#using-boxt-to-get-a-recursive-type-with-a-known-size].
/// The second element in the Cons variant is now RefCell<Rc<List>>, meaning that instead of
/// having the ability to modify the i32 value as we did in Listing 15-24, we want to modify
/// which List value a Cons variant is pointing to. We’re also adding a tail method to make it
/// convenient for us to access the second item if we have a Cons variant.

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // create the cycle here, a's tail points to b,    b -> a -> b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());
}