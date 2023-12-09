mod utils;

use std::collections::HashMap;
use std::ptr::hash;
use regex::Regex;
use utils::*;

fn part1() {
    let mut result = 0u32;
    for game in get_lines_nonempty("day04") {
        let num_regex = Regex::new(r#" ([0-9]*): "#).unwrap();
        let cc = num_regex.captures(game.as_str()).unwrap();
        let game_num = cc.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winning_regex = Regex::new(r#": (.*)\|"#).unwrap();
        let winning_string = winning_regex.captures(game.as_str()).unwrap().get(1).unwrap().as_str();
        let own_regex = Regex::new(r#"\|(.*)$"#).unwrap();
        let own_string = own_regex.captures(game.as_str()).unwrap().get(1).unwrap().as_str();
        let winning_numbers = winning_string
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let own_numbers = own_string
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut card_sum = 0u32;
        for i in own_numbers {
            if winning_numbers.contains(&i) {
                if card_sum == 0 {
                    card_sum = 1;
                }
                else {
                    card_sum *= 2;
                }
            }
        }
        result += card_sum;
    }
    println!("{}", result);
}

fn part2() {
    let mut result = 0u32;
    let mut extra_copies: HashMap<u32, u32> = HashMap::new();
    for game in get_lines_nonempty("day04") {
        let num_regex = Regex::new(r#" ([0-9]*): "#).unwrap();
        let cc = num_regex.captures(game.as_str()).unwrap();
        let game_num = cc.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winning_regex = Regex::new(r#": (.*)\|"#).unwrap();
        let winning_string = winning_regex.captures(game.as_str()).unwrap().get(1).unwrap().as_str();
        let own_regex = Regex::new(r#"\|(.*)$"#).unwrap();
        let own_string = own_regex.captures(game.as_str()).unwrap().get(1).unwrap().as_str();
        let winning_numbers = winning_string
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let own_numbers = own_string
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut card_sum = 0u32;
        let mut wins = 0;
        for i in own_numbers {
            if winning_numbers.contains(&i) {
                wins += 1;
                if card_sum == 0 {
                    card_sum = 1;
                }
                else {
                    card_sum *= 2;
                }
            }
        }
        let current_card_instances = 1 + *extra_copies.get(&game_num).unwrap_or(&0);
        for card_id_won in (game_num+1)..(game_num+1+wins) {
            let mut existing = *extra_copies.get(&card_id_won).unwrap_or(&0);
            existing += current_card_instances;
            extra_copies.insert(card_id_won, existing);
        }
        println!("game id {}: instances {}, sum {}", game_num, current_card_instances, card_sum * current_card_instances);
        result += current_card_instances;
    }
    println!("{}", result);
}

fn main() {
    //part1();
    part2();
}
