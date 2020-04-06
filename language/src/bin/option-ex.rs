/// Option examples

fn main() {
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    // Option::None maps to None
    let none: Option<String> = None;
    let map_to_none = none.map(|elem| elem.len() );
    assert_eq!(maybe_some_len, Some(13));
    assert_eq!(map_to_none, None);
}