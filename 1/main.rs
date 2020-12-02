use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(&args);
    println!("In file {}", filename);

    let lines = read_lines(filename);
    let numbers = get_numbers(lines.unwrap());
    /*for num in numbers {
        print!("{},", num);
    }
    println!("");*/
    
    let target = 2020;
    let pairs = two_sum(&numbers, target);
    for pair in pairs {
            println!("{} * {} = {}", pair.0, pair.1, pair.0 * pair.1); 
    } 

    let triplets = three_sum(&numbers, target);
    for triple in triplets {
            println!("{} * {} * {} = {}", triple.0, triple.1, triple.2, triple.0 * triple.1 * triple.2); 
    } 
}

fn parse_args(args: &[String]) -> &str {
    let filename = &args[1];
    return filename;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let mut numbers = Vec::new();
    for line in lines {
            if let Ok(number) = line {
                numbers.push(number.parse::<i32>().unwrap());
        }
    }
    return numbers;
}


fn two_sum(numbers: &Vec<i32>, target: i32) -> Vec<(i32, i32)> {
    let mut pairs = Vec::<(i32, i32)>::new();
    let mut map = HashMap::<i32, i32>::new();
    for number in numbers {
        if map.contains_key(&number) {
            map.insert(*number, map[&number] + 1);
        } else {
            map.insert(*number, 1);
        }
    }

    for number in numbers {
        if target - number == *number && map[&number] >= 2 {
            pairs.push((*number, *number));
        }
         else {
             if map.contains_key(&(target - number)) {
                 pairs.push((*number, target-*number));
             }
        }

    }
    return pairs;
}

fn three_sum(numbers: &Vec<i32>, target: i32) -> Vec<(i32, i32, i32)> {
    let mut triplets = Vec::<(i32, i32, i32)>::new();
    let mut map = HashMap::<i32, i32>::new();
    for number in numbers {
        if map.contains_key(&number) {
            map.insert(*number, map[&number] + 1);
        } else {
            map.insert(*number, 1);
        }
    }

    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            let num1 = numbers[i];
            let num2 = numbers[j];
            if map.contains_key(&(target - num1 - num2)) {
                triplets.push((num1, num2, target - num1 - num2));
            }
        }
    }
    return triplets;
}


