use anyhow::{anyhow, Result};
use std::fs::read_to_string;

fn part1(input: impl Into<String>) -> Result<i64> {
    let mut sum: i64 = 0;
    let input_str: String = input.into();

    for line in input_str.lines() {
        let dimensions: Vec<i64> = line
            .split('x')
            .map(|dimension| {
                dimension
                    .parse::<i64>()
                    .expect("dimension should parse to an integer")
            })
            .collect();

        if dimensions.len() != 3 {
            return Err(anyhow!("Each line should specify 3 dimensions"));
        }

        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
        let surface_area = vec![(2 * l * w), (2 * w * h), (2 * h * l)];

        sum += surface_area.iter().sum::<i64>();
        sum += surface_area
            .iter()
            .min()
            .expect("should always have a minimum")
            / 2;
    }

    Ok(sum)
}

fn part2(input: impl Into<String>) -> Result<i64> {
    let mut sum: i64 = 0;
    let input_str: String = input.into();

    for line in input_str.lines() {
        let dimensions: Vec<i64> = line
            .split('x')
            .map(|dimension| {
                dimension
                    .parse::<i64>()
                    .expect("dimension should parse to an integer")
            })
            .collect();

        if dimensions.len() != 3 {
            return Err(anyhow!("Each line should specify 3 dimensions"));
        }

        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
        let half_perimeter = vec![l + w, w + h, h + l];

        sum += half_perimeter
            .iter()
            .min()
            .expect("should always have a minimum")
            * 2;

        sum += l * w * h;
    }

    Ok(sum)
}

fn main() {
    let input = read_to_string("input").expect("should read input file");

    println!("AoC 2025, Day 2!");
    println!("Part 1: {:?}", part1(&input).unwrap());
    println!("Part 2: {:?}", part2(&input).unwrap());
}

#[test]
fn test_part1() {
    assert_eq!(part1("2x3x4").unwrap(), 58);
    assert_eq!(part1("1x1x10").unwrap(), 43);
}

#[test]
fn test_part2() {
    assert_eq!(part2("2x3x4").unwrap(), 34);
    assert_eq!(part2("1x1x10").unwrap(), 14);
}
