use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../input").unwrap();
    let mut reader = BufReader::new(file);
    let mut grid = vec![vec![]];
    let mut result = 0;
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        if let Ok(0) = len {
            break;
        }
        let line = line
            .trim()
            .bytes()
            .map(|b| b == b'@')
            .collect::<Vec<bool>>();
        grid.push(line);
    }
    for a in 0..grid.len() {
        for b in 0..grid[a].len() {
            if !grid[a][b] {
                continue;
            }
            let a = a as i32;
            let b = b as i32;
            let mut adjacent_roll_count = 0;
            for c in -1..=1 {
                for d in -1..=1 {
                    if (c, d) == (0, 0) {
                        continue;
                    }
                    if grid
                        .get((a + c) as usize)
                        .is_some_and(|l| l.get((b + d) as usize).is_some_and(|l| *l))
                    {
                        adjacent_roll_count += 1;
                    }
                }
            }
            if adjacent_roll_count < 4 {
                result += 1;
            }
        }
    }
    println!("{result}");
}
