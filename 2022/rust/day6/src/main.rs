use std::collections::HashSet;
use std::fs::read_to_string;

fn first_marker_position(datastream: &str, marker_length: usize) -> Option<usize> {
    if datastream.len() < marker_length {
        return None;
    }

    for (index, _char) in datastream[..datastream.len() - 1 - marker_length].char_indices() {
        if datastream[index..index + marker_length]
            .chars()
            .collect::<HashSet<char>>()
            .len()
            == marker_length
        {
            return Some(index + marker_length);
        }
    }

    None
}

fn main() {
    let input = read_to_string("../input/day6").expect("should always find input file");

    println!("Advent of Code 2022, Day 6!");
    println!("Part 1: {:?}", first_marker_position(input.trim(), 4));
    println!("Part 2: {:?}", first_marker_position(input.trim(), 14));
}

#[test]
fn test_part1() {
    vec![
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ]
    .iter()
    .for_each(|(example, marker)| assert_eq!(first_marker_position(example, 4).unwrap(), *marker));
}

#[test]
fn test_part2() {
    vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ]
    .iter()
    .for_each(|(example, marker)| assert_eq!(first_marker_position(example, 14).unwrap(), *marker));
}
