// day 05
const INPUTS: &str = include_str!("inputs.txt");

use std::collections::HashSet;

use crate::utils::AocDay;
pub struct DayFive;

fn get_middle_nums_ok() -> u64 {
    let mut cnt = 0u64;

    let (rules, updates) = parse_file();
    for u in updates {
        if is_upd_ok(&rules, &u).is_empty() {
            cnt += u[u.len() / 2] as u64;
        }
    }

    cnt
}

fn parse_file() -> (HashSet<String>, Vec<Vec<i32>>) {
    let mut rules = HashSet::new();
    let mut updates = Vec::new();

    let mut rules_done = false;

    for line in INPUTS.trim().split("\n") {
        if line.is_empty() {
            rules_done = true;
            continue;
        }
        if !rules_done {
            rules.insert(line.to_owned());
        } else {
            updates.push(
                line.trim()
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }
    (rules, updates)
}

fn is_upd_ok(rules: &HashSet<String>, update: &Vec<i32>) -> Vec<String> {
    let mut violations = Vec::new();
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            let inverse_rule = format!("{}|{}", update[j], update[i]);
            if rules.contains(&inverse_rule) {
                violations.push(inverse_rule);
            }
        }
    }
    violations
}

fn sort_wrong_updates() -> u64 {
    let mut cnt = 0u64;

    let (rules, updates) = parse_file();
    for u in updates {
        if !is_upd_ok(&rules, &u).is_empty() {
            let nu = sort_update(&rules, u);
            cnt += nu[nu.len() / 2] as u64;
        }
    }

    cnt
}

fn sort_update(rules: &HashSet<String>, update: Vec<i32>) -> Vec<i32> {
    let mut nu = update.clone();

    loop {
        let violations = is_upd_ok(rules, &nu);
        if violations.is_empty() {
            return nu;
        };
        for violation in violations {
            let parts = violation
                .split('|')
                .flat_map(str::parse)
                .collect::<Vec<i32>>();
            let a = &parts[0];
            let b = &parts[1];

            let a_index = nu.iter().position(|c| c == a).unwrap();
            let b_index = nu.iter().position(|c| c == b).unwrap();

            nu.swap(a_index, b_index);
        }
    }
}

impl AocDay for DayFive {
    fn part1() {
        println!("day 4 - part 1: {}", get_middle_nums_ok());
    }

    fn part2() {
        println!("day 4 - part 2: {}", sort_wrong_updates());
    }
}
