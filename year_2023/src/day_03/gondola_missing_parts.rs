use std::collections::BTreeMap;
use std::fs;
use itertools::Itertools;

/**
 * Advent of code
 * Day 03
 * https://adventofcode.com/2023/day/3
 * Gondola missing parts
 */

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

fn process_part_01(input: &str) -> String {
    // sum part numbers
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(
                move |(x, character)| {
                    (
                        (y as i32,x as i32),
                        match character {
                            '.' => Value::Empty,
                            c if c.is_ascii_digit() => {
                                Value::Number(
                                    c.to_digit(10).expect(
                                        "should be a number",
                                    ),
                                )
                            }
                            c => Value::Symbol(c),
                        },
                    )
                },
            )
        })
        .collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers
                                    .iter_mut()
                                    .last()
                                    .expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num,
                                )]);
                            }
                        }
                        None => unimplemented!(
                            "shouldn't happen"
                        ),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }

    // map: entire grid
    // numbers: sequential numbers
    let mut total = 0;
    for num_list in numbers {
        // (x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let num_positions: Vec<(i32, i32)> = num_list
            .iter()
            .map(|((y, x), _)| (*x, *y))
            .collect();
        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    // outer_pos.x + pos.x, .y + .y
                    (
                        outer_pos.0 + pos.1,
                        outer_pos.1 + pos.0,
                    )
                })
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        // dbg!(pos_to_check.len(), pos_to_check);
        let is_part_number =
            pos_to_check.iter().any(|pos| {
                let value = map.get(pos);
                #[allow(clippy::match_like_matches_macro)]
                if let Some(Value::Symbol(_)) = value {
                    true
                } else {
                    false
                }
            });

        if is_part_number {
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
    }

    total.to_string()
}

fn process_part_02(input: &str) -> String {
    // sum part numbers
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(
                move |(x, character)| {
                    (
                        (y as i32,x as i32),
                        match character {
                            '.' => Value::Empty,
                            c if c.is_ascii_digit() => {
                                Value::Number(
                                    c.to_digit(10).expect(
                                        "should be a number",
                                    ),
                                )
                            }
                            c => Value::Symbol(c),
                        },
                    )
                },
            )
        })
        .collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers
                                    .iter_mut()
                                    .last()
                                    .expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num,
                                )]);
                            }
                        }
                        None => unimplemented!(
                            "shouldn't happen"
                        ),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }

    // map: entire grid
    // numbers: sequential numbers
    let mut total = 0;
    for symbol in map.iter().filter(|(_key, value)| {
        matches!(value, Value::Symbol('*'))
    }) {
        // (x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let pos_to_check: Vec<(i32, i32)> = positions
            .iter()
            .map(|outer_pos| {
                // outer_pos.x + pos.x, .y + .y
                (
                    outer_pos.0 + symbol.0 .1,
                    outer_pos.1 + symbol.0 .0,
                )
            })
            .collect();

        // dbg!(pos_to_check.len(), pos_to_check);
        let mut indexes_of_numbers = vec![];

        for pos in pos_to_check {
            for (i, num_list) in numbers.iter().enumerate()
            {
                if num_list
                    .iter()
                    .any(|(num_pos, _)| num_pos == &pos)
                {
                    indexes_of_numbers.push(i);
                }
            }
        }

        let is_gear =
            indexes_of_numbers.iter().unique().count() == 2;

        if is_gear {
            total += indexes_of_numbers
                .iter()
                .unique()
                .map(|index| {
                    numbers[*index]
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .product::<usize>();
        }
    }

    total.to_string()
}

pub fn part_01() {
    let file: String = fs::read_to_string("./src/day_03/input_01.txt").unwrap();
    println!("Sum of missing parts for day 01 is {}", process_part_01(file.as_str()));
}

pub fn part_02() {
    let file: String = fs::read_to_string("./src/day_03/input_02.txt").unwrap();
    println!("Sum of missing parts for day 02 is {}", process_part_02(file.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_01: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    const INPUT_02: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";


    #[test]
    fn test_process_part_01() {
        assert_eq!("4361", process_part_01(INPUT_01));
    }

    #[test]
    fn test_process_part_02() {
        assert_eq!("467835", process_part_02(INPUT_02));
    }
}