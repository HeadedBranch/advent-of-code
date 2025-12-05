use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../test").unwrap();
    let mut reader = BufReader::new(file);
    let mut invalid_ids: Vec<u64> = vec![];
    let mut input = String::new();
    let _  = reader.read_line(&mut input);
    let input = input.trim().split(',');
    for range in input {
        let mut range = range.split('-');
        let start = range.next().unwrap().parse::<u64>().unwrap();
        let end = range.next().unwrap().parse::<u64>().unwrap();
        for i in start..=end {
            let str = i.to_string();
            if str.len() % 2 == 1 {
                continue;
            }
            let factor: u64 = 10_u64.pow(str.len() as u32 / 2) + 1;
            if i % factor == 0 {
                invalid_ids.push(i);
            }
        }
    }
    let result: u64 = invalid_ids.iter().sum();
    println!("{result}");
}
