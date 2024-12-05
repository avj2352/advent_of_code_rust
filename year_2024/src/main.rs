mod day_01;
use std::fs;

// my solution, worked 😎
fn process_part_02(input: &str) -> String {
    let mut left = vec![];
    let mut right = vec![];
    let mut dup_list: Vec<(i32, i32)> = vec![];
    let mut sum: i32 = 0;


    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
        right.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
    }

    for item in left {
        dup_list.push((item, right.iter().filter(|&&n| n == item).count() as i32));        
    }

    for (k,v) in dup_list {
        sum += k * v;
    }

    sum.to_string()


}

fn process_part_01(input: &str) -> String {
    // let output: i32 = input
        // println!("Input: {}", input);
        let mut left = vec![];
        let mut right = vec![];
    
        for line in input.lines() {
            let mut items = line.split_whitespace();
            left.push(
                items.next().unwrap().parse::<i32>().unwrap(),
            );
            right.push(
                items.next().unwrap().parse::<i32>().unwrap(),
            );
        }
    
        left.sort();
        right.sort();
    
        let result: i32 = std::iter::zip(left, right)
            .map(|(l, r)| (l - r).abs())
            .sum();   
        result.to_string()
}

pub fn part_01() {
    let file: String = fs::read_to_string("./src/day_01/hh_sample.txt").unwrap();
    println!("Historian Hysteria #1 sum value is: {}", process_part_01(file.as_str()));
}


pub fn part_02() {
    let file: String = fs::read_to_string("./src/day_01/hh_01.txt").unwrap();
    println!("Historian Hysteria #2 similarity score is: {}", process_part_02(file.as_str()));
}

fn main() {
    part_01();
    part_02();
}
