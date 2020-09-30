

fn main() {
    let mut s = String::from("Hello World");
    let offset = s.find(" ").expect("space not found");
    println!("space offset in Hello World is {}", offset);
    // replace a range in a String
    s.replace_range(5..6, "ZZZ");
    println!("{}", s);


    // appending Strings, can use += if you use &str
    let mut s2 = String::from("quick brown fox");
    let mut s3 = String::from(" jumped over a dog");
    s2.push_str(&s3);
    println!("s2 = {}", &s2);
    println!("s3 = {}", &s3);
    s3 += " append more chars";
    println!("s3 is now {}", &s3);
    println!("s2 is now {}", &s2);

    // concatenating a slice onto a String
    let mut str1 = String::from("this is a test");
    let sli2 = " of slices";
    str1.push_str(sli2);
    println!("String concat slice= {}", str1);

    // inserting into strings
    let v = String::from("abcd");
    println!("v length {} (in bytes)", v.len());
    for i in 0..=v.len() {
        let mut v1 = v.clone();
        v1.insert_str(i, "()");
        println!("v1 {}", v1);
    }

    let mut s = String::from("()");
    let mut i: usize = 0;
    for n in 0..2 {
        s.insert_str(n, "()");
        i += 2;
    }
    println!("final {}", s);
}