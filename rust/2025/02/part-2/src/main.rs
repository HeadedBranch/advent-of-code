use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);
    let mut invalid_ids: Vec<u64> = vec![];
    let mut input = String::new();
    let _  = reader.read_line(&mut input);
    let input = input.trim().split(',');
    for range in input {
        let mut range = range.split('-');
        let start = range.next().unwrap().parse::<u64>().unwrap();
        let end = range.next().unwrap().parse::<u64>().unwrap();
        'outer: for i in start..=end {
            for a in 1..=i.to_string().len()/2 {
                if i.to_string().len() % a != 0 {
                    continue;
                }
                if i.to_string().bytes().collect::<Vec<u8>>().windows(a).enumerate().filter(|(b,_)|b%a==0).map(|(_,b)|b).all(|s| *s == i.to_string().bytes().collect::<Vec<u8>>()[..a]) {
                    invalid_ids.push(i);
                    continue 'outer;
                }
            }
        }
    }
    let result: u64 = invalid_ids.iter().sum();
    println!("{result}");
}
