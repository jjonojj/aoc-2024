// day 05
const INPUTS: &str = include_str!("inputs.txt");

use crate::utils::AocDay;
use regex::Regex;
pub struct DayFive;

fn get_middle_nums_ok() -> u64 {
    let cnt = 0u64;

    cnt
}

fn parse_file() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let rule_re = Regex::new(r"(\d+\|\d+)").unwrap();
    let update_re = Regex::new(r"(\d+,?)+").unwrap();

    let mut rules_done = false;

    for line in INPUTS.trim().split("\n") {
        if line.is_empty() {
            rules_done = true;
            continue;
        }
        if !rules_done {
            let nums = rule_re.find_iter(line).collect::<Vec<_>>();
            assert!(nums.len() == 2);
            let a: i32 = nums[0].as_str().parse().unwrap();
            let b: i32 = nums[1].as_str().parse().unwrap();
            rules.push((a, b));
        } else {
            // tbd
            todo!()
        }
    }
    (rules, updates)
}

impl AocDay for DayFive {
    fn part1() {
        println!("day 4 - part 1: {}", get_middle_nums_ok());
    }

    fn part2() {
        println!("day 4 - part 2: {}", 0);
    }
}
