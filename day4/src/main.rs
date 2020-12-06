// day4
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Passport = HashMap<String, String>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = get_lines(&args[1]);
    let mut passports: Vec<Passport> = Vec::new();
    passports.push(Passport::new());
    let mut passport: &mut Passport = passports.last_mut().unwrap();
    for (_count, line) in lines.iter().enumerate() {
        //println!("{}", _count);
        if line.is_empty() {
            passports.push(Passport::new());
            passport = passports.last_mut().unwrap();
        } else {
            parse_line(line, passport);
        }
    }
    let mut valid_count_1 = 0;
    let mut valid_count_2 = 0;
    for p in passports {
        if validate_passport_1(&p) {
            valid_count_1 += 1;
        }
        if validate_passport_2(&p) {
            valid_count_2 += 1;
        }
    }
    println!("Valid passport count 1: {}", valid_count_1);
    println!("Valid passport count 2: {}", valid_count_2);
}

fn parse_line(line: &str, passport: &mut Passport) {
    line.split(" ")
        .map(|l| l.split(":").collect::<Vec<&str>>())
        .for_each(|v| {
            passport.insert(v[0].to_string(), v[1].to_string());
        });
}

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect(&(filename.to_owned() + " not found!"));
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn validate_passport_1(passport: &Passport) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for key in required_keys {
        if !passport.contains_key(key) {
            return false;
        }
    }
    true
}

fn validate_hgt(hgt: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>in|cm)$").unwrap();
    }
    let capture_result = RE.captures(hgt);
    if capture_result.is_none() {
        return false;
    }
    let caps = capture_result.unwrap();

    if caps.name("num").is_none() {
        return false;
    }
    if caps.name("unit").is_none() {
        return false;
    }
    let num = caps
        .name("num")
        .map_or("", |m| m.as_str())
        .parse::<i32>()
        .unwrap();
    let unit = caps.name("unit").map_or("", |m| m.as_str());
    match unit {
        "cm" => (150..194).contains(&num),
        "in" => (59..77).contains(&num),
        _ => false,
    }
}

fn validate_hcl(hcl: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    !RE.captures(hcl).is_none()
}

fn validate_pid(pid: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    !RE.captures(pid).is_none()
}

fn validate_passport_2(passport: &Passport) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    //println!("");
    //for (k, v) in passport {
    //println!("{}: {}", k, v);
    //}
    let ecl_options: HashSet<&'static str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();
    for key in required_keys {
        if !passport.contains_key(key) {
            //println!("{} missing, check failed!", key);
            return false;
        }
        match key {
            "byr" => {
                // [1920, 2002]
                if !(1920..2003).contains(&passport[key].parse::<i32>().unwrap()) {
                    //println!("{} check failed!", key);
                    return false;
                }
            }
            "iyr" => {
                // [2010, 2020]
                if !(2010..2021).contains(&passport[key].parse::<i32>().unwrap()) {
                    //println!("{} check failed!", key);
                    return false;
                }
            }
            "eyr" => {
                // [2020, 2030]
                if !(2020..2031).contains(&passport[key].parse::<i32>().unwrap()) {
                    //println!("{} check failed!", key);
                    return false;
                }
            }
            "hgt" => {
                if !validate_hgt(&passport[key]) {
                    //println!("{} check failed!", key);
                    return false;
                }
            }
            "hcl" => {
                if !validate_hcl(&passport[key]) {
                    //println!("{} check failed!", key);
                    return false;
                }
            }
            "ecl" => {
                if !ecl_options.contains(&passport[key].as_str()) {
                    //println!("{} check failed!", key);
                    return false;
                }
            }
            "pid" => {
                if !validate_pid(&passport[key]) {
                    //println!("{} check failed!", key);
                    return false;
                }
            }
            _ => {
                // Do nothing.
            }
        }
    }
    //println!("Valid passport.");
    true
}
