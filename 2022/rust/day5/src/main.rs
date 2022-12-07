#![feature(iter_collect_into)]
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input/day5").expect("should always find input file");

    let (input_crates, input_moves) = input
        .trim()
        .split_once("\n\n")
        .expect("crates and move inputs seperated by empty line");

    let crates = parse_crates(input_crates);
    let moves = parse_moves(input_moves);

    println!("Advent of Code 2022, Day 5!");
    println!(
        "Part 1: {}",
        last_crates(moves_part1(crates.clone(), moves.clone()))
    );
    println!("Part 2: {}", last_crates(moves_part2(crates, moves)));
}

fn parse_stack_count(input_crates: &str) -> usize {
    (input_crates.lines().next().expect("first line").len() + 1) / 4
}

fn parse_crates(input_crates: &str) -> Vec<Vec<char>> {
    let mut crates: Vec<Vec<char>> = vec![vec![]; parse_stack_count(input_crates)];

    for line in input_crates.lines().rev().skip(1) {
        for (stack_pos, crate_pos) in (1..line.len() - 1).step_by(4).enumerate() {
            let crate_at_pos = line
                .chars()
                .nth(crate_pos)
                .expect("should have crate letter in pos");

            if crate_at_pos != ' ' {
                crates[stack_pos].push(crate_at_pos);
            }
        }
    }

    crates
}

fn parse_moves(input_moves: &str) -> Vec<(usize, usize, usize)> {
    let mut moves = vec![];

    input_moves.lines().for_each(|line| {
        let mut line = line
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<usize>().expect("should be numbers"));

        let (amount, from, to) = (
            line.next().expect("should have amount"),
            line.next().expect("should have from pos") - 1,
            line.next().expect("should have to pos") - 1,
        );

        moves.push((amount, from, to));
    });

    moves
}

fn moves_part1(crates: Vec<Vec<char>>, moves: Vec<(usize, usize, usize)>) -> Vec<Vec<char>> {
    let mut crates = crates;

    moves.into_iter().for_each(|(amount, from, to)| {
        (0..amount).for_each(|_| {
            let moved_crate = crates[from].pop().expect("should not be empty");
            crates[to].push(moved_crate);
        })
    });

    crates
}

fn moves_part2(crates: Vec<Vec<char>>, moves: Vec<(usize, usize, usize)>) -> Vec<Vec<char>> {
    let mut crates = crates;
    let stack_count = crates.len();

    moves.into_iter().for_each(|(amount, from, to)| {
        let move_crates_after = (crates[from].len() - amount)..;
        let mut moved_crates: Vec<char> = crates[from].drain(move_crates_after).collect();
        crates[to].append(&mut moved_crates);
    });

    crates
}

fn last_crates(crates: Vec<Vec<char>>) -> String {
    crates
        .into_iter()
        .map(|stack| stack.into_iter().last().unwrap_or(' '))
        .collect()
}
