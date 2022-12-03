use std::{fs::read_to_string, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Error, Debug)]
enum MoveError {
    #[error("Invalid move: {0}; Allowed moves are A, B, C or X, Y, Z")]
    InvalidMove(String),
}

fn main() {
    let input = read_to_string("../input/day2").expect("should always find input file");

    let mut score1: u64 = 0;
    let mut score2: u64 = 0;

    input.trim().split('\n').for_each(|round| {
        let (player1, player2) = round.split_once(' ').expect("should have two players");
        let player1 = player1.parse::<Move>().expect("to be a valid move");
        let player2 = player2.parse::<Move>().expect("to be a valid move");

        score1 += round_score(player1, player2);

        score2 += player2 as u64;
        // score2 += match player1 {
        //     Move::Rock => 0,
        //     Move::Paper => 3,
        //     Move::Scissors => 6,
        // };
    });

    println!("Advent of Code 2022, Day 2!");
    println!("Part 1: {:?}", score1);
    // println!("Part 2: {:?}", score2);
}

fn round_score(opponent: Move, player: Move) -> u64 {
    let mut score: u64 = player as u64;

    if (player == Move::Rock && opponent == Move::Scissors)
        || (player == Move::Paper && opponent == Move::Rock)
        || (player == Move::Scissors && opponent == Move::Paper)
    {
        score += 6;
    }

    if player == opponent {
        score += 3;
    }

    score
}

impl FromStr for Move {
    type Err = MoveError;

    fn from_str(s: &str) -> Result<Self, MoveError> {
        let s_move = match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,

            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,

            _ => return Err(MoveError::InvalidMove(s.into())),
        };

        Ok(s_move)
    }
}
