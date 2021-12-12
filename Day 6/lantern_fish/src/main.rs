use std::time::Instant;
fn part1_slow(fish_ages: &mut Vec<usize>) {
    for _day in 0..80 {
        for i in 0..fish_ages.len() {
            if fish_ages[i] == 0 {
                fish_ages[i] = 6;
                fish_ages.push(8);
            } else {
                fish_ages[i] -= 1;
            }
        }
    }

    println!("There are {} lanternfish", fish_ages.len());
}

fn breed(fishes: &mut[u32], starting_index: usize, factor: u32) {
    let mut breeding_index = starting_index;
    while breeding_index < fishes.len() {
        fishes[breeding_index] += factor;
        breeding_index += 7;
    }
}

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let mut fish_ages: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1_slow(&mut fish_ages);
    println!("Part 1 took {:?}", start.elapsed());
}
