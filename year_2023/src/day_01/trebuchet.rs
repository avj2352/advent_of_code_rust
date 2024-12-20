use std::fs;

/***
 * Contains day 01 - part 01 and part 02 problem
 * solutions. Uses simple string parsing with
 * itertools & fs::read_from_string()
 */

// day_02: process function
fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|idx| {
        let reduced_line = &line[idx..];
        let result = match reduced_line {
            stmt if stmt.starts_with("one") => Some(1),
            stmt if stmt.starts_with("two") => Some(2),
            stmt if stmt.starts_with("three") => Some(3),
            stmt if stmt.starts_with("four") => Some(4),
            stmt if stmt.starts_with("five") => Some(5),
            stmt if stmt.starts_with("six") => Some(6),
            stmt if stmt.starts_with("seven") => Some(7),
            stmt if stmt.starts_with("eight") => Some(8),
            stmt if stmt.starts_with("nine") => Some(9),
            _ => reduced_line.chars().next().unwrap().to_digit(10),
        };        
        result
    });
    let first = it.next().expect("should be a number");
    match it.last() {
        Some(val) => first * 10 + val,
        None => first * 10 + first
    }
}

// day_01: process function
fn process_part_01(input: &str) -> String {
    let output: u32 = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character|{
                character.to_digit(10)
            });
            let first = it.next().expect("should be a number");
            match it.last() {
                Some(val) => first * 10 + val,
                None => first * 10 + first
            }
        })
        .sum::<u32>();
    output.to_string()
}

// day_02: process function
fn process_part_02(input: &str) -> String {
    let output: u32 = input
        .lines()
        .map(process_line)
        .sum::<u32>();
    output.to_string()
}




pub fn part_01() {
    let file = fs::read_to_string("./src/day_01/input_01.txt").unwrap();
    println!("Trebuchet #1 sum value is: {} ✅", process_part_01(file.as_str()));
}

pub fn part_02() {
    let file = fs::read_to_string("./src/day_01/input_02.txt").unwrap();
    println!("Trebuchet #2 new sum value is: {} ✅", process_part_02(file.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_01: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT_02: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    
    #[test]
    fn test_process_part_01() {
        assert_eq!("142", process_part_01(INPUT_01));
    }

    #[test]
    fn test_process_part_02() {
        assert_eq!("281", process_part_02(INPUT_02));
    }
}