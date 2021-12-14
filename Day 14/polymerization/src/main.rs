use std::time::Instant;
use std::cmp;

fn part2() {
}

fn part1() {
}

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");


    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1();
    println!("Part 1 took {:?}", start.elapsed());

    let start = Instant::now();
    part2();
    println!("Part 2 took {:?}", start.elapsed());
}
