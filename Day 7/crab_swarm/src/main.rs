use std::time::Instant;

fn part1(sorted_crab_locations: &Vec<i32>) {
    let mut fuel_required = 0;
    for crab_location in sorted_crab_locations {
        fuel_required += *crab_location - sorted_crab_locations[0];
    }

    let mut min_fuel = fuel_required;
    let mut index = 1;
    while index < sorted_crab_locations.len() {
        fuel_required = fuel_required + ((sorted_crab_locations[index] - sorted_crab_locations[index - 1]) * ((2 * index as i32) - sorted_crab_locations.len() as i32));
        if fuel_required < min_fuel {
            min_fuel = fuel_required;
        }
        index += 1;
    }

    println!("Minimum Fuel for linear crab subs is {}", min_fuel);
}

fn part2(sorted_crab_locations: &Vec<i32>) {
    let mut min_fuel = std::i32::MAX;
    for location in sorted_crab_locations[0]..=*sorted_crab_locations.last().expect("") {
        let mut fuel_cost = 0;
        for crab_location in sorted_crab_locations {
            let n = (location - crab_location).abs();
            fuel_cost += (n.pow(2) + n)/2;
        }
        min_fuel = std::cmp::min(min_fuel, fuel_cost);
    }

    println!("Minimum Fuel for exponential crab subs is {}", min_fuel);
}

fn main() {
    let input = include_str!("../input.txt");
    let mut crab_locations: Vec<i32> = input
        .lines()
        .next()
        .expect("Improper Input")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    crab_locations.sort();
    let start = Instant::now();
    part1(&crab_locations);
    println!("Part 1 took {:?}", start.elapsed());
    let start = Instant::now();
    part2(&crab_locations);
    println!("Part 2 took {:?}", start.elapsed());
}
