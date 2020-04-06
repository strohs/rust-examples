
use std::fmt::Display;

/// Lifetime annotations describe the relationships of the lifetimes of multiple references to
/// each other without affecting the lifetimes
///
/// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return
/// values of functions. Once they’re connected, Rust has enough information to allow memory-safe
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

// The constraint we want to express in this signature is that all the references in the
// parameters and the return value must have the same lifetime.
// The function signature now tells Rust that for some lifetime 'a, the function takes two
// parameters, both of which are string slices that live at least as long as lifetime 'a. The
// function signature also tells Rust that the string slice returned from the function will live
// at least as long as lifetime 'a. These constraints are what we want Rust to enforce. Remember,
// when we specify the lifetime parameters in this function signature, we’re not changing the
// lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker
// should reject any values that don’t adhere to these constraints.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return
// type, but not for the parameter y, because the lifetime of y does not have any relationship
// with the lifetime of x or the return value.
fn longest_always_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        //result = longest(string1.as_str(), string2.as_str());
        result = longest_always_x(string1.as_str(), string2.as_str());

    }
    println!("The longest string is {}", result);
}