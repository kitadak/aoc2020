use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = get_lines(&args[1]);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "Total trees: {}",
        slopes
            .into_iter()
            .map(|(a, b)| (count_trees(&lines, a, b)))
            .fold(1, |acc, x| acc * x as i64)
    );
}

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect(&(filename.to_owned() + " not found!"));
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn count_trees(lines: &Vec<String>, right: usize, down: usize) -> i32 {
    let mut total_count = 0;

    let mut x = 0;
    let mut y = 0;
    let mut curr = lines.iter();
    while y < lines.len() {
        let line = &curr.as_slice()[0];
        let curr_char = line.chars().nth(x).unwrap();
        if curr_char == '#' {
            total_count += 1;
        }
        // Move right.
        x = (x + right) % line.chars().count();
        // Increment iterator down times.
        for _ in 0..down {
            y += 1;
            curr.next();
        }
    }

    total_count
}
