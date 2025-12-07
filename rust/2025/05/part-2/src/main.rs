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
        let mut line = line.trim().split('-');
        let range_start = line.next().unwrap().parse::<u64>().unwrap();
        let range_end = line.next().unwrap().parse::<u64>().unwrap();
        ranges.push((range_start, range_end));
    }
    let mut result = 0;
    let mut count = 0;
    let ranges = loop {
        let mut change = false;
        let mut checked_ranges: Vec<(u64, u64)> = Vec::new();
        'a: for a in &ranges {
            for b in &mut checked_ranges {
                count += 1;
                println!("{:?}{:?}{}",a,b,count);
                if is_inside(a.0, b) && is_inside(a.1, b) {
                    // skip if range is already covered
                    continue 'a;
                }
                if is_inside(b.0, a) && is_inside(b.1, a) {
                    b.0 = a.0;
                    b.1 = a.1;
                    change = true;
                    continue 'a;
                }
                if is_inside(a.0, b) && !is_inside(a.1, b) {
                    b.1 = a.1;
                    change = true;
                    continue 'a;
                }
                if !is_inside(a.0, b) && is_inside(a.1, b) {
                    b.0 = a.0;
                    change = true;
                    continue 'a;
                }
            }
            checked_ranges.push(*a);
        }
        if !change {
            break checked_ranges;
        }
        ranges = checked_ranges;
    };
    for i in &ranges {
        result += (i.0..=i.1).count();
    }
    println!("{result}");
}

fn is_inside(num: u64, range: &(u64, u64)) -> bool {
    num >= range.0 && num <= range.1
}
