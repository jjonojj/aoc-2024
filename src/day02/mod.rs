// day 2.

use crate::utils::AocDay;
const FILE: &str = include_str!("inputs.txt");

pub struct DayTwo;

fn get_levels() -> Vec<String> {
    FILE.trim()
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
}

fn check_reports() -> u32 {
    let mut cnt_ok = 0u32;
    for line in get_levels() {
        let levels: Vec<u8> = line
            .trim()
            .split(" ")
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        if report_safe(&levels) {
            cnt_ok += 1;
        }
    }

    cnt_ok
}

fn check_reports_dampened() -> u32 {
    let mut cnt_ok = 0u32;
    for line in get_levels() {
        let levels: Vec<u8> = line
            .trim()
            .split(" ")
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        if report_safe(&levels) {
            cnt_ok += 1;
        } else {
            for idx in 0..levels.len() {
                let mut nl = levels.clone();
                nl.remove(idx);
                if report_safe(&nl) {
                    cnt_ok += 1;
                    break;
                }
            }
        }
    }

    cnt_ok
}

fn report_safe(report: &Vec<u8>) -> bool {
    let dir = report[0] < report[1];

    for idx in 0..report.len() - 1 {
        let cd = report[idx] < report[idx + 1];
        let diff = (report[idx] as i16 - report[idx + 1] as i16).abs();
        if cd != dir || diff > 3 || diff < 1 {
            return false;
        }
    }
    true
}

impl AocDay for DayTwo {
    fn part1() {
        println!("day 2 - part 1: {}", check_reports());
    }

    fn part2() {
        println!("day 2 - part 2: {}", check_reports_dampened());
    }
}
