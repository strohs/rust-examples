/// Lifetime annotations describe the relationships of the lifetimes of multiple references to
/// each other without affecting the lifetimes
///
/// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and **return
/// values of functions**. Once they’re connected, Rust has enough information to allow memory-safe
/// operations and disallow operations that would create dangling pointers or otherwise violate
/// memory safety.
///
/// ## Lifetime Ellision Rules
/// The compiler uses three rules to figure out what lifetimes references have when there aren’t
/// explicit annotations. The first rule applies to input lifetimes, and the second and third
/// rules apply to output lifetimes. If the compiler gets to the end of the three rules and there
/// are still references for which it can’t figure out lifetimes, the compiler will stop with an error
///
/// 1. each parameter that is a reference gets its own lifetime parameter. In other words, a
/// function with one parameter gets one lifetime parameter:
/// `fn foo<'a>(x: &'a i32);`
/// a function with two parameters gets two separate lifetime parameters:
/// `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.
///
/// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output
/// lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`
///
/// 3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`
/// because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.
/// This third rule makes methods much nicer to read and write because fewer symbols are necessary.
///
/// ## Lifetime Annotations in Method Definitions
/// 

// This annotation, 'a , means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 3rd lifetime ellision rule applies. There are two input lifetimes, so Rust applies the
    // first lifetime elision rule and gives both &self and announcement their own lifetimes. Then,
    // because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // The data in novel exists before the ImportantExcerpt instance is created. In addition,
    // novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the
    // reference in the ImportantExcerpt instance is valid
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    let fw = first_word(novel.as_str());
    println!("{}", fw);
}
