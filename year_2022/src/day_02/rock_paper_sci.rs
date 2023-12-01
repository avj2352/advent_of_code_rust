use std::fs;
use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Move::Scissor && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissor {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
            _ => Err("Not a known move".to_string()),
        }
    }
}

fn process_part_01(input: &str) -> String {
    let result: u32 = input.lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split_whitespace()
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => {
                    3 + moves[1] as u32
                },
                Some(Ordering::Greater) => {
                    0 + moves[1] as u32
                },
                Some(Ordering::Less) => {
                    6 + moves[1] as u32
                },
                None => panic!("moves should be comparable")
            }
        })
        .sum();
    result.to_string()
}

fn process_part_02(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<&str> =
                line.split_whitespace().collect();
            let opponent_move =
                moves[0].parse::<Move>().unwrap();
            match moves[1] {
                "X" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Scissor,
                        Move::Paper => Move::Rock,
                        Move::Scissor => Move::Paper,
                    };
                    0 + our_move as u32
                }
                "Y" => 3 + opponent_move as u32,
                "Z" => {
                    let our_move = match opponent_move {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissor,
                        Move::Scissor => Move::Rock,
                    };
                    6 + our_move as u32
                }
                _ => {
                    panic!("Invalid move");
                }
            }
        })
        .sum();
    result.to_string()
}

pub fn part_01() {
    let file = fs::read_to_string("./src/day_02/game.txt").unwrap();
    println!("Part1: rock paper scissor winner score is: {}", process_part_01(file.as_str()));
}

pub fn part_02() {
    let file = fs::read_to_string("./src/day_02/game.txt").unwrap();
    println!("Part2: rock paper scissor winner score is: {}", process_part_02(file.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_process_part_01() {
        assert_eq!("15", process_part_01(INPUT));
    }

    #[test]
    fn test_process_part_02() {
        assert_eq!("12", process_part_02(INPUT));
    }
}