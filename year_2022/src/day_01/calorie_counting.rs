use std::fs;
fn process_part_01(input: &str) -> String {
    let result = input.split("\n\n")
        .map(|elf_load| {
            elf_load
                // or .split("\n")
                .lines()
                // you can use unwrap in scenarios where you know result is well-formed
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

fn process_part_02(input: &str) -> String {
    let mut result: Vec<u32> = input.split("\n\n")
        .map(|elf_load| {
            elf_load
                // or .split("\n")
                .lines()
                // you can use unwrap in scenarios where you know result is well-formed
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    result.sort_by(|a,b| b.cmp(a));
    let total: u32 = result.iter().take(3).sum();
    total.to_string()
}

pub fn part_01() {
    let file = fs::read_to_string("./src/day_01/input.txt").unwrap();
    println!("elf carrying the highest calorie: {}", process_part_01(file.as_str()));
}

pub fn part_02() {
    let file = fs::read_to_string("./src/day_01/input.txt").unwrap();
    println!("elf carrying the highest calorie: {}", process_part_02(file.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_process_part_01() {
        assert_eq!("24000", process_part_01(INPUT));
    }

    #[test]
    fn test_process_part_02() {
        assert_eq!("45000", process_part_02(INPUT));
    }
}