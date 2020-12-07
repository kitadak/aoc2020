use std::collections::HashMap;
//use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Answers = HashMap<char, i32>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = get_lines(&args[1]);
    let mut groups: Vec<Answers> = Vec::new();
    groups.push(Answers::new());
    let mut answers: &mut Answers = groups.last_mut().unwrap();
    for (_count, line) in lines.iter().enumerate() {
        //println!("{}", _count);
        if line.is_empty() {
            groups.push(Answers::new());
            answers = groups.last_mut().unwrap();
        } else {
            parse_line(line, answers);
        }
    }
    let total_1: usize = groups
        .iter()
        .filter(|a| a.len() != 0)
        // Remove '#'.
        .map(|a| a.len() - 1)
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    println!("Sum of answers 1: {}", total_1);

    let total_2: usize = groups
        .iter()
        .filter(|a| a.len() != 0)
        .map(|a| match_count(a) - 1)
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    println!("Sum of answers 2: {}", total_2);
}

// Returns total number of questions that matched the count.
fn match_count(answers: &Answers) -> usize {
    let count = answers.get(&'#').unwrap();
    answers.iter().filter(|(_, b)| b == &count).count()
}

fn parse_line(line: &str, answers: &mut Answers) {
    //println!("answers size: {} ", answers.len());
    line.chars().for_each(|c| {
        if !answers.contains_key(&c) {
            answers.insert(c, 0);
        }
        *answers.get_mut(&c).unwrap() += 1;
    });
    // Added '#' as a character to keep track of # of people.
    if !answers.contains_key(&'#') {
        answers.insert('#', 0);
    }
    *answers.get_mut(&'#').unwrap() += 1;
}

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect(&(filename.to_owned() + " not found!"));
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
