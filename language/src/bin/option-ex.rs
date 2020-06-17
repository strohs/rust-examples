

/// Option examples


fn option_like_result(o: Option<i32>) -> Option<i32> {
    // will immediately return None if o is None
    let r1 = o?;

    if r1 > 10 {
        Some(20)
    } else {
        Some(99)
    }
}

fn main() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    // Option::None maps to None
    let none: Option<String> = None;
    let map_to_none = none.map(|elem| elem.len() );
    assert_eq!(maybe_some_len, Some(13));
    assert_eq!(map_to_none, None);

    // try out using ? with Option(s)
    let res = option_like_result(None);
    println!("res is {:?}", res);
}