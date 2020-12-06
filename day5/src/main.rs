use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = get_lines(&args[1]);
    let results: HashSet<i32> = lines.iter().map(|e| find_seat_id(e)).collect();
    println!("Max seat ID: {}", results.iter().max().unwrap());
    println!("My seat ID: {}", find_my_seat_id(&results));
}

fn find_my_seat_id(ids: &HashSet<i32>) -> i32 {
    // Assuming my seat has id s, s-1 and s+1 exist, but not s.
    // Then for each element e, check that e+2 exists but not e+1, return e+1;
    for id in ids {
        if ids.contains(&(id + 2)) && !ids.contains(&(id + 1)) {
            return id + 1;
        }
    }
    panic!("My seat doesn't exist!");
}

fn find_seat_id(pass: &str) -> i32 {
    let row = search(&pass[0..7], 0, 127);
    let col = search(&pass[7..10], 0, 8);
    (row * 8) + col
}

fn search(pass: &str, start: i32, end: i32) -> i32 {
    let mut low = start;
    let mut high = end;
    for c in pass.chars() {
        match c {
            'F' | 'L' => {
                high = (low + high) / 2;
            }
            'B' | 'R' => {
                low = (low + high + 1) / 2;
            }
            _ => {
                panic!("Illegal character found: {}", c);
            }
        }
    }
    low
}

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect(&(filename.to_owned() + " not found!"));
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .filter(|l| !l.is_empty())
        .collect()
}
