use std::fs;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{iterator, value},
    // multi::many1,
    IResult,
};

fn numbers(input: &str) -> IResult<&str, Option<u32>> {
    let res: IResult<&str, u32> = alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input);

    let (input, digit) = anychar(input)?;

    match res {
        Ok((_, digit)) => Ok((input, Some(digit))),
        Err(_) => Ok((input, digit.to_digit(10))),
    }
}

fn parser(input: &str) -> IResult<&str, Vec<u32>> {
    // can do this more simply than iterator, but it
    // costs some microseconds it
    // let (input, output) = many1(numbers)(input)?;
    let mut it = iterator(input, numbers);

    let output = it.flatten().collect();
    let (input, _) = it.finish()?;

    Ok((input, output))
}

fn process_line_part_02_nom(input: &str) -> u32 {
    let result = parser(input)
                                        .expect("should be a list of numbers");

    let mut it = result.1.iter();
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }    
}

pub fn process_part_02_nom(input: &str) -> String {
    let output: u32 = input
        .lines()        
        .map(|line| process_line_part_02_nom(line))
        .sum();
    output.to_string()
}

/***
 * Contains day 01 - part 02 problem
 * solution using nom parser.
 */
pub fn part_02_nom() {
    let file = fs::read_to_string("./src/day_01/input_02.txt").unwrap();
    println!("Trebuchet #2 sum value using nom parser is: {} ✅", process_part_02_nom(file.as_str()));
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_02: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test_process_part_02_using_nom() {
        assert_eq!("281", process_part_02_nom(INPUT_02));
    }

    #[test]
    fn test_single_digit_edge_case() {
        assert_eq!(11, process_line_part_02_nom("one"));
    }
}