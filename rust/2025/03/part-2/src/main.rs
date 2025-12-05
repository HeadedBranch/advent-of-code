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
        let mut i = (1..=9).rev();
        let mut furthest_index = 0;
        'outer: while 12 > number.len() {
            let cmp_val = match i.next() {
                Some(v) => v,
                None => break,
            };
            let result = loop {
                if furthest_index >= line.len() - (12 - number.len()) {
                    break None;
                }
                if let Some(pos) = line.iter().position(|b| *b == cmp_val) {
                    if pos > line.len() - number.len()-1 {
                        if pos < furthest_index {
                            line[pos] = 0;
                            continue;
                        }
                        continue 'outer;
                    }
                    furthest_index = pos;
                    line[pos] = 0;
                    break Some((pos,cmp_val));
                }
            };
            match result {
                Some((pos,val)) => number.push((pos,val)),
                None => break,
            }
        }
        println!("{:?}",number);
        number.append(&mut line.clone().into_iter().enumerate().collect::<Vec<(usize,u8)>>()[line.len()-(12-number.len())..].to_vec());
        println!("{:?}",number);
        number.sort();
        println!("{:?}", number);
        let mut num = 0;
        for (_,i) in number {
            print!("{i}");
            num *= 10;
            num += i as u64;
        }
        result += num;
        println!();
    }
    println!("{result}");
}

