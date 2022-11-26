// --- Day 2: Dive! ---

use std::fs;

pub fn solution() -> (u64, u64) {
    let file = fs::read_to_string("2.txt").expect("Something went wrong reading the file");
    let data: Vec<(&str, u64)> = file
        .lines()
        .map(|i| {
            let mut line = i.split(" ");
            let op = line.next().unwrap();
            let val = line.next().unwrap().parse::<u64>().unwrap();
            (op, val)
        })
        .collect();

    let mut pos = 0;
    let mut aim = 0;
    let mut dth = 0;

    for &(op, val) in &data {
        match op {
            "down" => aim += val,
            "up" => aim -= val,
            "forward" => {
                pos += val;
                dth += aim * val;
            }
            _ => panic!("Unknown operation"),
        }
    }

    (pos * aim, pos * dth)
}
