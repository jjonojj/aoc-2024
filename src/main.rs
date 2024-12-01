use std::{collections::HashMap, io::Write, process::exit};

mod day01;

fn main() {
    let mut map: HashMap<u8, (fn(), fn())> = HashMap::new();
    // put days here
    map.insert(1, (day01::part1, day01::part2));
    let mut input = String::new();

    print!("enter day:number > ");

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    let nums = input.trim().split(":").collect::<Vec<_>>();
    assert!(nums.len() > 1);
    let part = nums.get(0).unwrap().parse::<i32>().unwrap() as u8;

    match nums.get(1).unwrap().parse::<i32>().unwrap() {
        1 => map
            .get(&part)
            .unwrap_or_else(|| {
                println!("day does not exist (yet?)");
                exit(1);
            })
            .0(),
        2 => map
            .get(&part)
            .unwrap_or_else(|| {
                println!("day does not exist (yet?)");
                exit(1);
            })
            .1(),
        _ => {
            println!("this part does not exist");
            exit(1);
        }
    }

    // i hate myself for the thing above
}
