
// A trait object points to both an instance of a type implementing our specified trait as well
// as a table used to look up trait methods on that type at runtime. We create a trait object by
// specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the "dyn"
// keyword, and then specifying the relevant trait.
// We can use trait objects in place of a generic or concrete type. Wherever we use a trait object,
// Rust’s type system will ensure at compile time that any value used in that context will
// implement the trait object’s trait. Consequently, we don’t need to know all the possible types
// at compile time.
// Trait objects aren’t as generally useful as objects in other languages: their specific purpose
// is to allow abstraction across common behavior.

use std::fmt::Debug;

trait Quack: Debug {
    fn quack(&self);
}

#[derive(Debug)]
struct Duck ();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

#[derive(Debug)]
struct RandomBird {
    is_a_parrot: bool
}

impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}


fn main() {
    let duck1  = Duck();
    let duck2  = RandomBird{is_a_parrot: false};
    let parrot = RandomBird{is_a_parrot: true};

    // vector of borrowed trait objects: &Quack
    let ducks: Vec<&Quack> = vec![&duck1,&duck2,&parrot];

    for d in &ducks {
        d.quack();
    }


}