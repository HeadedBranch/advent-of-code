use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);
    let mut ranges = Vec::new();
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(1) = len {
            break;
        }
        let mut line= line.trim().split('-');
        let range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_end = line.next().unwrap().parse::<u64>().unwrap();
        ranges.push((range_start, range_end));
    }
    let mut result = 0;
    'outer: loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(0) = len {
            break;
        }
        let line = line.trim().parse::<u64>().unwrap();
        for i in &ranges {
            if line >= i.0 && line <= i.1 {
                result += 1;
                continue 'outer;
            }
        }
    }
    println!("{result}");
}

