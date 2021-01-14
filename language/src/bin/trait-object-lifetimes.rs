use std::fmt;
use std::fmt::Formatter;

/// example 19-19  from Rust Book Chapter 19.2
///
/// This code compiles without any errors, even though we haven’t explicitly annotated the
/// lifetimes involved in obj. This code works because there are rules for working with lifetimes
/// and trait objects:
///  * The default lifetime of a trait object is 'static.
///  * With &'a Trait or &'a mut Trait, the default lifetime of the trait object is 'a.
///  * With a single T: 'a clause, the default lifetime of the trait object is 'a.
///  * With multiple clauses like T: 'a, there is no default lifetime; we must be explicit.

trait Red: fmt::Display {
    fn desc(&self) -> String;
}

#[derive(Debug)]
struct Ball<'a> {
    diameter: &'a i32,
}
impl fmt::Display for Ball<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("ball:{}", self.diameter))
    }
}

#[derive(Debug)]
struct Square<'a> {
    side: &'a i32,
}
impl fmt::Display for Square<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("square:{}", self.side))
    }
}

#[derive(Debug)]
struct Sphere<'a> {
    surface_area: &'a i32,
}
impl fmt::Display for Sphere<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("sphere:{}", self.surface_area))
    }
}

struct RedList {
    reds: Vec<Box<dyn Red>>,
}
impl RedList {
    fn new() -> Self {
        Self { reds: vec![] }
    }
    fn add(&mut self, obj: impl Red + 'static) {
        self.reds.push(Box::new(obj));
    }
}

impl<'a> Red for Ball<'a> {
    fn desc(&self) -> String {
        format!("desc ball {}", self.diameter)
    }
}

impl<'a> Red for Square<'a> {
    fn desc(&self) -> String {
        format!("desc square {}", self.side)
    }
}

impl Red for Sphere<'_> {
    fn desc(&self) -> String {
        format!("desc sphere {}", self.surface_area)
    }
}

fn main() {
    //let num = 5;
    // the "as Box<dyn Red>" used to be required
    //let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
    // we can no omit it?
    //let obj = Box::new(Ball { diameter: &num });
    //let obj2 = Box::new( Sphere { surface_area: &57i32 });
    //obj.desc();
    //obj2.desc();

    // test adding traits to a Vec of traits
    let obj1 = Ball { diameter: &11 };
    let obj2 = Square { side: &22 };
    let obj3 = Sphere { surface_area: &33 };
    let mut reds_list = RedList::new();
    reds_list.add(obj1);
    reds_list.add(obj2);
    reds_list.add(obj3);
    reds_list.reds.iter().for_each(|r| println!("{}", r));
}