use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();

    }
    println!("{result}");
}

