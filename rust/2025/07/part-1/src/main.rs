use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);
    let mut beam_cols = HashSet::new();
    let mut result = 0;
    {
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        beam_cols.insert(
            line.bytes()
                .collect::<Vec<u8>>()
                .iter()
                .position(|c| *c == b'S')
                .unwrap(),
        );
    }
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(0) = len {
            break;
        }
        let mut line = line.bytes().collect::<Vec<u8>>();
        while let Some(pos) = line.iter().position(|c| *c == b'^') {
            if beam_cols.contains(&pos)
            {
                result += 1;
                beam_cols.insert(pos - 1);
                beam_cols.insert(pos + 1);
                beam_cols.remove(&pos);
                line[pos] = b'.';
            } else {
                line[pos] = b'.';
            }
        }
    }
    println!("{result}");
}
