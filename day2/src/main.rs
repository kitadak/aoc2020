#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(&args);
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut valid_count_1 = 0;
    let mut valid_count_2 = 0;
    let mut total_count = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        total_count += 1;
        if validate_line(&l) {
            valid_count_1 += 1;
        }
        if validate_line_2(&l) {
            valid_count_2 += 1;
        }
    }
    println!("Valid 1 password count: {}/{}", valid_count_1, total_count);
    println!("Valid 2 password count: {}/{}", valid_count_2, total_count);
}

fn parse_args(args: &[String]) -> &str {
    let filename = &args[1];
    return filename;
}

fn validate_line(line: &String) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<target>[a-z]): (?P<password>[a-z]+)")
                .unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let min = caps
        .name("min")
        .map_or("", |m| m.as_str())
        .parse::<i32>()
        .unwrap();
    let max = caps
        .name("max")
        .map_or("", |m| m.as_str())
        .parse::<i32>()
        .unwrap();
    let target = caps.name("target").map_or("", |m| m.as_str());
    let password = caps.name("password").map_or("", |m| m.as_str());
    let count = password.matches(target).count() as i32;
    let is_valid = count >= min && count <= max;
    //println!(
    //"min: {} \nmax: {}\ntarget: {}\npassword: {}\nis_valid: {}",
    //min, max, target, password, is_valid
    //);
    //println!("occurences: {}\n", password.matches(target).count());

    is_valid
}

fn validate_line_2(line: &String) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<target>[a-z]): (?P<password>[a-z]+)")
                .unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let first = caps
        .name("min")
        .map_or("", |m| m.as_str())
        .parse::<usize>()
        .unwrap();
    let second = caps
        .name("max")
        .map_or("", |m| m.as_str())
        .parse::<usize>()
        .unwrap();
    let target = caps.name("target").map_or("", |m| m.as_str());
    let password = caps.name("password").map_or("", |m| m.as_str());
    let in_first = password.chars().nth(first - 1) == target.chars().nth(0);
    let in_second = password.chars().nth(second - 1) == target.chars().nth(0);
    //println!("first: {}, second: {}", in_first, in_second);
    in_first ^ in_second
}
