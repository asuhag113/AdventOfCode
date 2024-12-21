use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    let (mut left, mut right) = (Vec::new(), Vec::new());   
    input.split("\n").for_each(|l| {
        let line = l.split_whitespace().collect::<Vec<_>>();
        let left_part: u32 = line[0].parse().unwrap();
        let right_part: u32 = line[1].parse().unwrap();
        left.push(left_part);
        right.push(right_part);
    });
    left.sort_unstable();
    right.sort_unstable();
    println!(
        "{}",
        left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u32>()
    );
}

fn part_two(input: &String) {
    let mut digit_to_count = HashMap::new();
    let mut left = Vec::new();   
    input.split("\n").for_each(|l| {
        let line = l.split_whitespace().collect::<Vec<_>>();
        let left_part: u32 = line[0].parse().unwrap();
        let right_part: u32 = line[1].parse().unwrap();
        left.push(left_part);
        digit_to_count.entry(right_part).and_modify(|counter| *counter += 1).or_insert(1);
    });
    println!("{}", left.iter().map(|l| l * digit_to_count.get(l).unwrap_or_else(|| &0)).sum::<u32>());
}
