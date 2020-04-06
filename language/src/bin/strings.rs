

fn main() {
    let mut s = String::from("Hello World");
    let offset = s.find(" ").expect("space not found");
    println!("space offset is {}", offset);
    s.replace_range(5..6, "ZZZ");
    println!("{}", s);

    // appending strings
    let mut s2 = String::from("quick brown fox");
    let s3 = String::from(" jumped over a dog");
    s2.push_str(&s3);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);
}