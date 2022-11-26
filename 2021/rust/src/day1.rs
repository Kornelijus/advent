// --- Day 1: Sonar Sweep ---

use std::fs;

pub fn solution() -> (u64, u64) {
    let data: Vec<u64> = fs::read_to_string("1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|i| i.parse::<u64>().unwrap())
        .collect();

    let increased = |d: &Vec<u64>| d.windows(2).fold(0, |a, i| a + (i[0] < i[1]) as u64);
    let part1 = increased(&data);
    let part2 = increased(&data.windows(3).map(|i| i.iter().sum()).collect());

    (part1, part2)
}
