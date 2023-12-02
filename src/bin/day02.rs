mod utils;

use std::collections::HashMap;
use utils::*;

fn part1() {
    let limits = HashMap::from([
        ("red", 12u32),
        ("green", 13u32),
        ("blue", 14u32),
    ]);

    let mut result = 0;
    for game in get_lines_nonempty("day02") {
        let parts = game.split(": ").collect::<Vec<&str>>();
        let p1 = parts.first().unwrap();
        let p2 = parts.last().unwrap();
        let num_char = *p1.split(" ").collect::<Vec<&str>>().last().unwrap();
        let mut game_num = num_char.parse::<u32>().unwrap();
        println!("{}", game);
        for draw in p2.split("; ").collect::<Vec<&str>>() {
            for ball_parts in draw.split(", ").collect::<Vec<&str>>() {
                println!("{}", ball_parts);
                let chunks = ball_parts.split(" ").collect::<Vec<&str>>();
                let ball_num = chunks.first().unwrap().parse::<u32>().unwrap();
                let color = *chunks.last().unwrap();
                if *limits.get(color).unwrap() < ball_num {
                    game_num = 0;
                    continue;
                }
            }
        }
        result += game_num;
        println!("{}", game);
    }
    println!("{}", result);
}

fn part2() {
    let mut result = 0;
    for game in get_lines_nonempty("day02") {
        let parts = game.split(": ").collect::<Vec<&str>>();
        let p1 = parts.first().unwrap();
        let p2 = parts.last().unwrap();
        let num_char = *p1.split(" ").collect::<Vec<&str>>().last().unwrap();
        let mut game_num = num_char.parse::<u32>().unwrap();
        println!("{}", game);
        let mut mins = HashMap::from([
            ("red", 0u32),
            ("green", 0u32),
            ("blue", 0u32),
        ]);
        for draw in p2.split("; ").collect::<Vec<&str>>() {
            for ball_parts in draw.split(", ").collect::<Vec<&str>>() {
                println!("{}", ball_parts);
                let chunks = ball_parts.split(" ").collect::<Vec<&str>>();
                let ball_num = chunks.first().unwrap().parse::<u32>().unwrap();
                let color = *chunks.last().unwrap();
                if *mins.get(color).unwrap() < ball_num {
                    mins.insert(color, ball_num);
                }
            }
        }
        let mult = *mins.get("red").unwrap() *
            *mins.get("green").unwrap() *
            *mins.get("blue").unwrap();
        result += mult;
    }
    println!("{}", result);
}

fn main() {
    //part1();
    part2();
}
