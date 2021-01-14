

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32
}

/// mix and match: `if let` with `if` and `else if`
fn if_let_examples() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/// while let loop using pattern matching
fn while_let_examples() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 | 5 | 8 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn range_patterns() {
    let x = 5;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn char_range_patterns() {
    let x = 'c';

    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn matching_structs() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn nested_structs_and_enums() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}

fn match_guard_examples() {
    let num = Some(4);
    let y = 10;

    match num {
        Some(x) if x == 4 && y <= 11 => println!("x is {} and y is{}", x,y),
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

// The at operator (@) lets us create a variable that holds a value at the same time we’re
// testing that value to see whether it matches a pattern
fn bindings_example() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10...12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

/// matching on parts of slice, The .. is called a "rest pattern," because it matches the rest of the slice
fn subslice_patterns() {
    let words = ["zHello","World","!","Run","Forrest"];
    match &words {
        ["Hello", "World", "!", ..] => println!("Hello World!"),
        ["Foo", "Bar", ..] => println!("Baz"),
        rest => println!("{:?}", rest),
    }
}

fn subslice_patterns2() {
    let words = ["hello","all","z"];
    match &words {
        // Ignore everything but the last element, which must be "!".
        [.., "!"] => println!("!!!"),

        // `start` is a slice of everything except the last element, which must be "z".
        [start @ .., "z"] => println!("starts with: {:?}", start),

        // `end` is a slice of everything but the first element, which must be "a".
        ["a", end @ ..] => println!("ends with: {:?}", end),

        rest => println!("{:?}", rest),
    }
}

/// This macro accepts an expression and a pattern, and returns true if the pattern matches the expression
fn matches_macro() {
    // Using the `matches!` macro:
    //matches!(self.partial_cmp(other), Some(Less));

    // You can also use features like | patterns and if guards:
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}

fn main() {
    subslice_patterns();
    subslice_patterns2();
    // if_let_examples();
    // while_let_examples();
    // char_range_patterns();
    // matching_structs();
    // bindings_example();

    // We can use patterns to destructure structs, enums, tuples, and references
    // destructuring structs
    let p = Point { x: 0, y: 7 };
    //let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);


    // complex destructuring
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    // ignoring parts of a value with ..
    let origin = Point3d {x:0, y:0, z:0};
    match origin {
        Point3d {x, ..} => println!("x is {}", x)
    }

    // ignoring parts of a tuple with ..
    let numbers = (2,4,8,12,32);
    match numbers {
        (first, .., last) => println!("first:{} last:{}", first, last)
        // using .. must be ambiguous, the following pattern will not compile
        // (.., second, ..)
    }

    // to match vectors, need to use experimental rust with #![feature(slice_patterns)]
//    let v1 = vec![1,3,5,7,9];
//    match &v1[..] {
//        [first, ..] => println!("first of v1:{}", first)
//    }
}