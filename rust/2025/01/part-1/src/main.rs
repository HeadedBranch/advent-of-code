use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);
    let mut position: i32 = 50;
    let mut count = 0;
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(0) = len {
            break
        }
        let mut line = line.bytes();
        let _ = line.next_back();
        let increase = line.next() == Some(b'R');
        let mut delta: i32 = 0;
        for x in line { delta *= 10; delta += i32::from(x) - 48 };
        if increase {
            position += delta
        } else {
            position -= delta
        }
        position %= 100;
        if position == 0 { count += 1 }
    };
    println!("{count}");
}
