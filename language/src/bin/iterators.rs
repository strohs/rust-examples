

fn str_ref( s: &str ) {
    println!("{}", &s[0..4]);
}

fn str_append( s: &mut String ) {
    s.push_str("TTTT");
}

fn iter_ex() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // borrowing iterator: iter
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us...{}", name),
            _ => println!("Hello {}", name),
        }
    }
}

fn opt_iter_ex() {
    let names = vec![Some("Bob"), Some("Frank"), Some("Ferris")];

    // borrowing iterator: iter
    for name in names.iter() {
        match name {
            Some("Ferris") => println!("There is a rustacean among us...{}", name.unwrap() ),
            Some(_) => println!("Hello {}", name.unwrap()),
            None => println!("NONE"),
        }
    }
    println!("names len {}", names.len());
}

fn into_iter_ex() {
    let names = vec!['a', 'b', 'c'];

    // consuming iterator: into_iter
    for name in names.into_iter() {
        match name {
            'b' => println!("There is a Char B among us!"),
            _ => println!("Hello {}", name),
        }
    }
    //println!("the resulting names vec{:?}", names);
}

fn iter_mut_ex() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    // mutable borrow  iter_mut:
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn main() {
    // string literals are string slices
    let hello = "hello world";
    // String slice range indices must occur at valid UTF-8 character boundaries. If you attempt
    // to create a string slice in the middle of a multibyte character, your program will exit with an error
    let ptr = &hello[0..3];

    // parsing strings
    let one_hundred: &str = "-100";
    let p100: i32 = one_hundred.parse().unwrap();
    println!("parsed 100 to {}",p100);
    let four = "4".parse::<u32>(); // using turbofish

    let s1: &str = "this is a string slice, it is part of rust core";
    // creates a String type from a string literal
    let s2: String = String::from("'String' is a growable, mutable, owned, UTF-8 encoded string type");
    // converting a string slice to a String type
    let mut data: String = s1.to_string();
    str_ref(s1);
    str_append(&mut data);
    println!("{}", data);
    opt_iter_ex();
}
