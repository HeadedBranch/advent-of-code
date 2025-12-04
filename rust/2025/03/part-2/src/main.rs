use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../test").unwrap();
    let mut reader = BufReader::new(file);
    let mut result: u64 = 0;
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(0) = len {
            break;
        }
        let mut line = line.trim().bytes().map(|x| x - b'0').collect::<Vec<u8>>();
        let mut number: Vec<(usize,u8)> = vec![];
        for _ in 0..12 {
            let mut i = (1..=9).rev();
            number.push(loop {
                let cmp_val = i.next().unwrap();
                if let Some(pos) = line.iter().rposition(|b| *b == cmp_val) {
                    if pos == line.len() - 1 && cmp_val != 1 {
                        continue;
                    } else {
                        line[pos] = 0;
                        break (pos, cmp_val);
                    }
                }
            });
        }
        number.sort();
        println!("{}",number.len());
        println!("{:?}", number);
        let mut num = 0;
        for (_,i) in number {
            num *= 10;
            num += i as u64;
        }
        result += num;
    }
    println!("{result}");
}

