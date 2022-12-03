use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input/day1").unwrap();

    let mut calories: Vec<u64> = input
        .trim()
        .split("\n\n")
        .map(|nums| {
            nums.split('\n')
                .map(|n| n.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect();

    calories.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let max_calories: u64 = calories[0];
    let top_3_max_calories: u64 = calories[0] + calories[1] + calories[2];

    println!("Advent of Code 2022, Day 1!");
    println!("Part 1: {:?}", max_calories);
    println!("Part 2: {:?}", top_3_max_calories);
}
