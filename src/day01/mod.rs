// day 01 - aoc

const INPUTS: &str = include_str!("inputs.txt");

fn parse() -> (Vec<i32>, Vec<i32>) {
    let mut pairs = (Vec::new(), Vec::new());
    for line in INPUTS.split("\n") {
        let pair_str = line.trim().split("   ").collect::<Vec<_>>();
        // very bad bug fix
        if pair_str.len() < 2 {
            continue;
        }
        pairs
            .0
            .push(pair_str.get(0).unwrap().parse::<i32>().unwrap());
        pairs
            .1
            .push(pair_str.get(1).unwrap().parse::<i32>().unwrap());
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

pub fn part1() {
    println!("part 1: {}", get_dists(parse()))
}

pub fn part2() {
    println!("part 2: {}", get_similarity(parse()))
}
