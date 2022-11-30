use std::fs::read_to_string;

fn part1(input: impl Into<String>) -> i64 {
    let mut floor: i64 = 0;
    let input_str: String = input.into();

    for char in input_str.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }

    floor
}

fn part2(input: impl Into<String>) -> i64 {
    let mut floor: i64 = 0;
    let input_str: String = input.into();

    for (i, char) in input_str.char_indices() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            return (i + 1) as i64;
        }
    }

    -1
}

fn main() {
    let input = read_to_string("input").expect("should read input file");

    println!("AoC 2025, Day 1!");
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

#[test]
fn test_part1() {
    let examples = vec![
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ];

    for (input, result) in examples {
        assert_eq!(part1(input), result)
    }
}

#[test]
fn test_part2() {
    assert_eq!(part2(")"), 1);
    assert_eq!(part2("()())"), 5);
}
