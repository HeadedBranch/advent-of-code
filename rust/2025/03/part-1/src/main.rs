use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);
    let mut result: u32 = 0;
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(0) = len {
            break;
        }
        let line = line.trim().bytes().map(|x| x - b'0').collect::<Vec<u8>>();
        let mut i = (1..=9).rev();
        let largest_pos = loop {
            let cmp_val = i.next().unwrap();
            if let Some(pos) = line.iter().position(|b| *b == cmp_val) {
                if pos == line.len() - 1 {
                    continue;
                } else {
                    break pos;
                }
            }
        };
        result += line[largest_pos] as u32 * 10;
        let line = &line[largest_pos + 1..];
        result += *line.iter().max().unwrap() as u32;
    }
    println!("{result}");
}
