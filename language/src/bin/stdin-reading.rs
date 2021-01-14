use std::io;
use std::io::{BufReader, BufRead};

fn read_input() -> io::Result<String> {
    let mut input = String::new();

    let mut br = BufReader::new( io::stdin() );

    let res = br.read_line(&mut input)?;

    println!("You typed: {} ({})", input.trim(), res);

    Ok(input)
}

fn main() {
    loop {
        match read_input() {
            Ok(ref s) if s == "Q\n" => break,
            Err(e) => println!("some error occurred {}", e),
            _ => ()
        }
    }
    println!("ending program")
}
