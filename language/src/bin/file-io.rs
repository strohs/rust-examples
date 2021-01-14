use std::fs;
use std::io::{ErrorKind, Read, BufRead};
use std::io;
use std::fs::File;
use std::path::Path;


fn main() {


}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn open_file() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

fn read_from_file_to_string_v1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file_to_string_v2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_from_file_to_string_v3(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn print_dir_names(p: &str) {
    let path = Path::new(p);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("DirEntry path: {:?}", entry.path());
            println!("  file_type: {:?}", entry.file_type().expect("file_type failed"));
            println!("  file_name: {:?}", entry.file_name()); // dir name
        }
    }
}