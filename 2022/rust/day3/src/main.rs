#![feature(iter_array_chunks)]

mod split_in_half;

use split_in_half::SplitInHalf;
use std::fs::read_to_string;

fn item_priority(item: char) -> u32 {
    match item {
        _ if ('a'..='z').contains(&item) => (item as u8 - b'a' + 1) as u32,
        _ if ('A'..='Z').contains(&item) => (item as u8 - b'A' + 27) as u32,
        _ => 0,
    }
}

fn main() {
    let input = read_to_string("../input/day3").expect("should always find input file");
    let lines = input.trim().split('\n');

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for rucksack in lines.clone() {
        let (first, second) = rucksack
            .split_in_half()
            .expect("for rucksack to have an even amount of characters");

        for item in rucksack.chars() {
            if first.contains(item) && second.contains(item) {
                part1 += item_priority(item);
                break;
            }
        }
    }

    for group in lines.array_chunks::<3>() {
        'rucksack: for rucksack in group {
            for item in rucksack.chars() {
                if group.iter().all(|rucksack| rucksack.contains(item)) {
                    part2 += item_priority(item);
                    break 'rucksack;
                }
            }
        }
    }

    println!("Advent of Code 2022, Day 3!");
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
