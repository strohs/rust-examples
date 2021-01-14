use std::fs;
use std::io::{ErrorKind, Read};
use std::io;
use std::path::Path;
use regex::{Regex, Captures};

fn main() {
    let path_str = "./";
    let findRegex = Regex::new(r"^(?P<prefix>\(3\)\s+)(?P<mag>[-a-zA-Z0-9]+) - (?P<model>\w+ \w+)\s+-\s+(?P<title>[a-zA-Z0-9 ]+).+").expect("invalid findRegex");
    find_and_print_dir_names(path_str, &findRegex);
}

fn find_and_print_dir_names(p: &str, findRx: &Regex) {
    let path = Path::new(p);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str().expect("could not convert dir name to string");
            let result = findRx.replace(&file_name_str, "$mag - $model - $title");
            println!("res is: {}", result);
        }
    }
}