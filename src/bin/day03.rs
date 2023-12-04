mod utils;

use std::collections::HashMap;
use std::ptr::{hash, null};
use utils::*;

fn part1() {
    let mut row = 0i32;
    let mut symbols_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let all_lines = get_lines_nonempty("day03");

    // Make a map of all the symbols.
    for line in all_lines.iter() {
        let enum_chars = line.chars().enumerate();
        let mut symbols = vec![];
        for (col, char) in enum_chars {
            if !char.is_numeric() && char != '.' {
                symbols.push(col as u32);
            }
        }
        symbols_map.insert(row as u32, symbols);
        row += 1;
    }

    // Check the numbers.
    let mut result = 0u32;
    row = 0;
    for line in all_lines {
        let mut number = "".to_string();
        let enum_chars = line.chars().enumerate();
        let mut reset = false;
        for (col, char) in enum_chars {
            if reset {
                number = "".to_string();
                reset = false;
            }
            let mut is_end_of_num = false;
            let mut is_start_of_num = false;

            // Check if the number ends here.
            if !char.is_numeric() {
                continue;
            }

            is_start_of_num = number.is_empty();
            number += char.to_string().as_str();
            match line.chars().nth(col + 1) {
                None => is_end_of_num = true,
                Some(ch) => {
                    if !ch.is_numeric() {
                        is_end_of_num = true;
                    }
                }
            }

            if !is_end_of_num {
                continue;
            }
            reset = true;
            if is_machine_part(&number, col as i32, row, &symbols_map) {
                println!("IS a part: {}: {}", line, number);
                result += number.parse::<u32>().unwrap();
            }
        }
        row += 1;
    }
    println!("{}", result);
}

fn is_machine_part(
    number: &String,
    col: i32,
    row: i32,
    symbols_map: &HashMap<u32, Vec<u32>>,
) -> bool {
    for (char_i, _) in number.chars().enumerate() {
        let col_i = col - number.chars().count() as i32 + 1 + char_i as i32;
        let is_start = char_i == 0;
        let is_end = char_i == (number.chars().count() - 1);
        let positions: Vec<(i32, i32)> = vec![
            (row - 1, col_i - 1),
            (row - 1, col_i),
            (row - 1, col_i + 1),
            (row + 1, col_i - 1),
            (row + 1, col_i),
            (row + 1, col_i + 1),
        ];
        for pos in positions {
            if is_symbol_at_pos(pos.0, pos.1, &symbols_map) {
                return true;
            }
        }
        if is_end && is_symbol_at_pos(row, col_i + 1, &symbols_map) {
            return true;
        }
        if is_start && is_symbol_at_pos(row, col_i - 1, &symbols_map) {
            return true;
        }
    }
    false
}

fn is_machine_part_gears(
    number: &String,
    col: i32,
    row: i32,
    symbols_map: &HashMap<u32, Vec<u32>>,
    gears: &mut HashMap<(u32, u32), Vec<u32>>,
) -> bool {
    for (char_i, _) in number.chars().enumerate() {
        let col_i = col - number.chars().count() as i32 + 1 + char_i as i32;
        let is_start = char_i == 0;
        let is_end = char_i == (number.chars().count() - 1);
        let positions: Vec<(i32, i32)> = vec![
            (row - 1, col_i - 1),
            (row - 1, col_i),
            (row - 1, col_i + 1),
            (row + 1, col_i - 1),
            (row + 1, col_i),
            (row + 1, col_i + 1),
        ];
        for pos in positions {
            if is_symbol_at_pos(pos.0, pos.1, &symbols_map) {
                return mark_gear(pos.0, pos.1, number, gears);
            }
        }
        if is_end && is_symbol_at_pos(row, col_i + 1, &symbols_map) {
            return mark_gear(row, col_i + 1, number, gears);
        }
        if is_start && is_symbol_at_pos(row, col_i - 1, &symbols_map) {
            return mark_gear(row, col_i - 1, number, gears);
        }
    }
    false
}

fn mark_gear(row: i32, col: i32, num: &String, gears: &mut HashMap<(u32, u32), Vec<u32>>) -> bool {
    let key = &(row as u32, col as u32);
    let mut existing: Vec<u32> = gears.get(key).unwrap_or(&vec![]).to_vec();
    existing.push(num.parse::<u32>().unwrap());
    gears.insert(*key, existing);
    true
}

fn is_symbol(ch: &char) -> bool {
    return !ch.is_numeric() && *ch != '.';
}

fn is_symbol_at_pos(row: i32, col: i32, symbols_map: &HashMap<u32, Vec<u32>>) -> bool {
    if row < 0 || col < 0 {
        return false;
    }
    let row_entry_option = symbols_map.get(&(row as u32));
    if row_entry_option.is_none() {
        return false;
    }
    let row_entry = row_entry_option.unwrap();
    return row_entry.contains(&(col as u32));
}

fn part2() {
    let mut gears: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut row = 0i32;
    let mut symbols_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let all_lines = get_lines_nonempty("day03");

    // Make a map of all the symbols.
    for line in all_lines.iter() {
        let enum_chars = line.chars().enumerate();
        let mut symbols = vec![];
        for (col, char) in enum_chars {
            if !char.is_numeric() && char != '.' {
                symbols.push(col as u32);
            }
        }
        symbols_map.insert(row as u32, symbols);
        row += 1;
    }

    // Check the numbers.
    let mut result = 0u32;
    row = 0;
    for line in all_lines {
        let mut number = "".to_string();
        let enum_chars = line.chars().enumerate();
        let mut reset = false;
        for (col, char) in enum_chars {
            if reset {
                number = "".to_string();
                reset = false;
            }
            let mut is_end_of_num = false;
            let mut is_start_of_num = false;

            // Check if the number ends here.
            if !char.is_numeric() {
                continue;
            }

            is_start_of_num = number.is_empty();
            number += char.to_string().as_str();
            match line.chars().nth(col + 1) {
                None => is_end_of_num = true,
                Some(ch) => {
                    if !ch.is_numeric() {
                        is_end_of_num = true;
                    }
                }
            }

            if !is_end_of_num {
                continue;
            }
            reset = true;
            if is_machine_part_gears(&number, col as i32, row, &symbols_map, &mut gears) {
                println!("IS a part: {}: {}", line, number);
                result += number.parse::<u32>().unwrap();
            }
        }
        row += 1;
    }
    let mut res = 0;
    for (_, gear) in gears {
        if gear.len() != 2 {
            continue;
        }
        res += gear.first().unwrap() * gear.last().unwrap();
    }
    println!("{}", res);
}

fn main() {
    //part1();
    part2();
}
