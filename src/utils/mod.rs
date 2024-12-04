pub trait AocDay {
    fn part1();
    fn part2();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point(pub i32, pub i32);
