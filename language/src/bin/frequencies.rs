use std::collections::HashMap;

/// Counts the number of times a character occurs in a string
///
/// # Returns
/// a HashMap of with a character as a key, and the number of times it occurs as the value
fn frequencies(s: &str) -> HashMap<char, u32> {
    let mut freqs: HashMap<char, u32> = HashMap::new();
    s.chars().for_each(|c| {
        let count = freqs.entry(c).or_insert(0);
        *count += 1;
    });
    freqs
}

fn main() {
    let str1 = "this is a journey into sound";

    let fr = frequencies(str1);

    for (k, v) in &fr {
        println!("{}::{}", k, v)
    }
}
