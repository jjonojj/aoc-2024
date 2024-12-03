// day 03 - aoc ( regex :c )

use regex::Regex;

use crate::utils::AocDay;
#[allow(dead_code)]
const INPUTS: &str = include_str!("inputs.txt");

pub struct DayThree;

pub fn count_muls() -> i64 {
    let mut sum = 0i64;

    let inp = INPUTS.replace("\n", "");
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap(); // mul({num}, {num})
    let w = Regex::new(r"\d{1,3}").unwrap();
    for mul in re.find_iter(&inp) {
        let ms = mul.as_str();
        let mut nums = w.find_iter(ms);
        let l: i16 = nums.next().unwrap().as_str().parse().unwrap();
        let r: i16 = nums.next().unwrap().as_str().parse().unwrap();
        sum += l as i64 * r as i64;
    }

    sum
}

pub fn count_muls_cond() -> i64 {
    let mut sum = 0i64;

    let inp = INPUTS.replace("\n", "");
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap(); // either mul({num}, {num}) or do() or don't()
    let w = Regex::new(r"\d{1,3}").unwrap();

    let mut collect = true;
    for mul in re.find_iter(&inp) {
        let ms = mul.as_str();
        match ms {
            "don't()" => collect = false,
            "do()" => collect = true,
            _ => {
                if !collect {
                    continue;
                }

                let mut nums = w.find_iter(ms);
                let l: i16 = nums.next().unwrap().as_str().parse().unwrap();
                let r: i16 = nums.next().unwrap().as_str().parse().unwrap();
                sum += l as i64 * r as i64;
            }
        }
    }

    sum
}

impl AocDay for DayThree {
    fn part1() {
        println!("day 3 - part 1: {}", count_muls());
    }

    fn part2() {
        println!("day 3 - part 2: {}", count_muls_cond());
    }
}
