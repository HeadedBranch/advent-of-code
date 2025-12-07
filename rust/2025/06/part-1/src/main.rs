use std::fs::File;
use std::io::{BufRead, BufReader};

struct Calculation {
    values:Vec<String>,
    calculation: String,
}
impl Calculation {
    fn calculate(&self) -> u64 {
        let values = self.values.iter().map(|v| v.parse::<u64>().unwrap());
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
        let line = line.split_whitespace().map(|s| s.to_string());
        lines.push(line.clone().collect::<Vec<String>>());
    };
    let mut calculations = Vec::new();
    for i in 0..lines[0].len() {
        calculations.push(Calculation {values: lines[0..lines.len()-1].iter().map(|r| r[i].clone()).collect(), calculation: lines[lines.len()-1][i].clone()})
    }
    for calculation in calculations {
        result += calculation.calculate();
    }
    println!("{result}");
}

