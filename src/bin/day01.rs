mod utils;

use std::collections::HashMap;
use utils::*;

fn part1() {
    let mut sum = 0;
    for line in get_lines("day01") {
        if line.is_empty() {
            continue;
        }
        let mut line_num = "".to_string();
        let mut last = "".to_string();
        for char in line.chars() {
            if !char.is_numeric() {
                continue;
            }
            if line_num.is_empty() {
                line_num += char.to_string().as_str();
                last = char.to_string();
                continue;
            }
            last = char.to_string();
        }
        line_num += last.as_str();
        println!("{}: {}", line, line_num);
        let line_num_num = line_num.parse::<u32>().unwrap();
        sum += line_num_num;
    }
    println!("{}", sum);
}

fn part2() {
    let mut sum = 0;
    for mut line in get_lines("day01") {
        let orig_lin = line.clone();
        if line.is_empty() {
            continue;
        }
        let mut line_num = "".to_string();
        let mut last = "".to_string();
        while !line.is_empty() {
            let val = pop_num_from_line(&mut line);
            if val.is_none() {
                continue;
            }
            let num = val.unwrap();
            if line_num.is_empty() {
                line_num += num.as_str();
            }
            last = num;
        }
        line_num += last.as_str();
        println!("{}: {}", orig_lin, line_num);
        let line_num_num = line_num.parse::<u32>().unwrap();
        sum += line_num_num;
    }
    println!("{}", sum);
}

fn pop_num_from_line(line: &mut String) -> Option<String> {
    let digits: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    if line.is_empty() {
        return None;
    }

    let char = line.chars().next().unwrap();
    if char.is_numeric() {
        *line = line.as_str()[1..].to_string();
        return Some(char.to_string());
    }

    let mut index = 2;
    while index <= line.len() {
        let substring = line.as_str()[..index].to_string();
        let num = digits.get(substring.as_str());
        match num {
            None => {
                index += 1;
            }
            Some(num_val) => {
                *line = line.as_str()[1..].to_string();
                return Some(num_val.to_string());
            }
        }
    }
    *line = line.as_str()[1..].to_string();
    None
}

fn main() {
    part2();
}
