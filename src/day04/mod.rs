// day 04 - crossword

const INPUTS: &str = include_str!("inputs.txt");

use crate::utils::{AocDay, Point};
pub struct DayFour;

struct Dir(i32, i32);

const DIRS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    /*  */ (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

// part 1
fn xmas_count() -> u64 {
    let mut cnt = 0;

    let xmasgrid = INPUTS
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut xs: Vec<Point> = Vec::new();
    INPUTS.trim().lines().enumerate().for_each(|(y, l)| {
        l.trim().chars().enumerate().for_each(|(x, c)| {
            if c == 'X' {
                xs.push(Point(x as i32, y as i32))
            }
        })
    });

    for xp in xs {
        for d in DIRS {
            cnt += (check_xmas(&xmasgrid, xp, Dir(d.0, d.1))) as u64
        }
    }

    cnt
}

fn check_xmas(grid: &Vec<Vec<char>>, pos: Point, dir: Dir) -> bool {
    let match_str = "MAS";
    let mut got_str = String::with_capacity(3);

    for (idx, wantc) in match_str.chars().enumerate() {
        if let Some(row) = grid.get((pos.1 as i32 + (dir.1 * (idx as i32 + 1))) as usize) {
            if let Some(gotc) = row.get((pos.0 as i32 + (dir.0 * (idx as i32 + 1))) as usize) {
                if *gotc == wantc {
                    got_str.push(wantc);
                }
            }
        }
    }

    match_str == got_str.as_str()
}

// part 2
fn x_mas_count() -> u64 {
    let mut cnt = 0;

    let xmasgrid = INPUTS
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut aps: Vec<Point> = Vec::new();
    INPUTS.trim().lines().enumerate().for_each(|(y, l)| {
        l.trim().chars().enumerate().for_each(|(x, c)| {
            if c == 'A' {
                aps.push(Point(x as i32, y as i32))
            }
        })
    });
    println!("a count: {}", ac);

    for ap in aps {
        cnt += check_x_mas(&xmasgrid, ap) as u64
    }

    cnt as u64
}

fn check_x_mas(grid: &Vec<Vec<char>>, pos: Point) -> bool {
    // first diagonal
    let mut m_count = 0;
    let mut s_count = 0;
    [(-1, 1), (1, -1)].iter().for_each(|(xo, yo)| {
        if let Some(row) = grid.get((pos.1 + yo) as usize) {
            if let Some(gotc) = row.get((pos.0 + xo) as usize) {
                match *gotc {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => (),
                }
            }
        }
    });

    // quick check
    if m_count != 1 || s_count != 1 {
        return false;
    }

    // second diagonal
    [(-1, -1), (1, 1)].iter().for_each(|(xo, yo)| {
        if let Some(row) = grid.get((pos.1 + yo) as usize) {
            if let Some(gotc) = row.get((pos.0 + xo) as usize) {
                match *gotc {
                    'M' => m_count += 1,
                    'S' => s_count += 1,
                    _ => (),
                }
            }
        }
    });

    if m_count == s_count && m_count == 2 {
        return true;
    }
    false
}

impl AocDay for DayFour {
    fn part1() {
        println!("day 4 - part 1: {}", xmas_count());
    }

    fn part2() {
        println!("day 4 - part 2: {}", x_mas_count());
    }
}
