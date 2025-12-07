use std::fs::File;
use std::io::{BufRead, BufReader};

struct Calculation {
    values:Vec<String>,
    calculation: String,
}
impl Calculation {
    fn calculate(&self) -> u64 {
        let values = self.values.iter().map(|v| v.trim().parse::<u64>().unwrap());
        if self.calculation == "+" {
            return values.sum();
        }
        values.product()
    }
}
fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut result = 0;
    let lines = loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(0) = len {
            break lines;
        }
        let line = line.chars();
        lines.push(line.clone().collect::<Vec<char>>());
    };
    let mut calculations = Vec::new();
    let mut iter = 0..lines[0].len()-1;
    let mut i = 0;
    loop {
        let mut values = Vec::new();
        let ret = loop {
            i = match iter.next() {
                Some(v) => v,
                None => {
                    i += 1;
                    break None
                },
            };
            if lines[0..lines.len()].iter().map(|r| r[i]).collect::<Vec<char>>() == vec![' '; lines.len()] {
                break Some(());
            }
            values.push(lines[0..lines.len() - 1].iter().map(|r| r[i]).collect::<String>());
        };
        let len = values.len();
        calculations.push(Calculation { values, calculation: String::from(lines[lines.len() - 1][i-len]) });
        if ret.is_none() {
            break;
        }
    }
    for calculation in calculations {
        let add = calculation.calculate();
        println!("{add}");
        result += add;
    }
    println!("{result}");
}

