// --- Day 6: Lanternfish ---

use std::fs;

const SPEED: usize = 6;
const NEW_SPEED: usize = SPEED + 2;

fn fish_sum_after(init_timers: &Vec<u64>, days: i32) -> u64 {
    let mut timers = init_timers.clone();

    for _ in 0..days {
        let today = timers[0];
        for day in 1..NEW_SPEED + 1 {
            timers[day - 1] = timers[day];
        }
        timers[NEW_SPEED] = today;
        timers[SPEED] += today;
    }

    timers.iter().sum()
}

pub fn solution() -> (u64, u64) {
    let file =
        fs::read_to_string("../inputs/6.txt").expect("Something went wrong reading the file");

    let data: Vec<u8> = file
        .trim()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let timers: Vec<u64> = (0..NEW_SPEED + 1)
        .map(|i| data.iter().filter(|j| **j == i as u8).count() as u64)
        .collect();

    (fish_sum_after(&timers, 80), fish_sum_after(&timers, 256))
}
