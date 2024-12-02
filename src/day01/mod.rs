// day 01 - aoc

use crate::utils::AocDay;
const INPUTS: &str = include_str!("inputs.txt");

pub struct DayOne;

fn parse() -> (Vec<i32>, Vec<i32>) {
    let mut pairs = (Vec::new(), Vec::new());
    for line in INPUTS.trim().split("\n") {
        let pair_str = line.trim().split("   ").collect::<Vec<_>>();
        pairs.0.push(pair_str[0].parse::<i32>().unwrap());
        pairs.1.push(pair_str[1].parse::<i32>().unwrap());
    }

    pairs.0.sort();
    pairs.1.sort();
    pairs
}

fn get_dists(parsed: (Vec<i32>, Vec<i32>)) -> u64 {
    let mut sum = 0u64;

    assert_eq!(parsed.0.len(), parsed.1.len());

    parsed
        .0
        .iter()
        .zip(parsed.1.iter())
        .for_each(|(x, y)| sum += x.abs_diff(*y) as u64);

    sum
}

fn get_similarity(parsed: (Vec<i32>, Vec<i32>)) -> u64 {
    let mut sum = 0u64;

    for num in parsed.0.iter() {
        sum += (num * parsed.1.iter().filter(|x| *x == num).count() as i32) as u64;
    }

    sum
}

impl AocDay for DayOne {
    fn part1() {
        println!("day 1 - part 1: {}", get_dists(parse()))
    }

    fn part2() {
        println!("day 1 - part 2: {}", get_similarity(parse()))
    }
}
