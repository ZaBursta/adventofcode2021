use std::collections::HashMap;
use std::time::Instant;
use std::cmp;

fn do_step(polymer: String, reaction_map: &HashMap<(char, char), char>) -> String {
    let mut output_polymer = polymer.chars().next().unwrap().to_string();
    for window in polymer.chars().collect::<Vec<char>>().windows(2) {
        if reaction_map.contains_key(&(window[0], window[1])) {
            output_polymer.push(reaction_map.get(&(window[0], window[1])).unwrap().to_owned());
        }
        output_polymer.push(window[1]);
    }

    return output_polymer;
}

fn do_step_memory_optimized(polymer: &HashMap<(char, char), u128>, reaction_map: &HashMap<(char, char), char>) -> HashMap<(char, char), u128> {
    let mut new_polymer: HashMap<(char, char), u128> = HashMap::new();

    for (pair, count) in polymer {
        if reaction_map.contains_key(pair) {
            let inserted_char = reaction_map.get(pair).unwrap();
            *new_polymer.entry((pair.0, *inserted_char)).or_insert(0) += count;
            *new_polymer.entry((*inserted_char, pair.1)).or_insert(0) += count;
        } else {
            *new_polymer.entry(*pair).or_insert(0) += count;
        }
    }
    return new_polymer;
}

fn find_most_and_least_common_char_count_in_string(string: String) -> (u32, u32) {
    let mut char_counts: [u32; 26] = [0; 26];
    for char in string.chars() {
        char_counts[(char as u32 - 65) as usize] += 1;
    }
    let mut min = u32::MAX;
    let mut max = u32::MIN;

    for char_count in char_counts {
        max = cmp::max(char_count, max);
        if char_count != 0 {
            min = cmp::min(char_count, min);
        }
    }

    return (max, min);
}

fn find_most_and_least_common_char_count_in_map(start_char: char, end_char: char, map: &HashMap<(char, char), u128>) -> (u128, u128) {
    let mut char_counts: [u128; 26] = [0; 26];

    for (pair, count) in map {
        char_counts[(pair.0 as u32 - 65) as usize] += count;
        char_counts[(pair.1 as u32 - 65) as usize] += count;
    }

    char_counts[(start_char as u32 - 65) as usize] += 1;
    char_counts[(end_char as u32 - 65) as usize] += 1;

    let mut min = u128::MAX;
    let mut max = u128::MIN;

    for char_count in char_counts {
        max = cmp::max(char_count/2, max);
        if char_count != 0 {
            min = cmp::min(char_count/2, min);
        }
    }

    return (max, min);
}

fn part2(start_polymer: &str, reaction_map: &HashMap<(char, char), char>) {
    let mut polymer: HashMap<(char, char), u128> = HashMap::new();
    for window in start_polymer.chars().collect::<Vec<char>>().windows(2) {
        *polymer.entry((window[0], window[1])).or_insert(0) += 1;
    }

    for _ in 0..40 {
        polymer = do_step_memory_optimized(&polymer, reaction_map);
    }

    start_polymer.chars().next().unwrap();
    let (most_common_count, least_common_count) = find_most_and_least_common_char_count_in_map(start_polymer.chars().next().unwrap(), start_polymer.chars().last().unwrap(), &polymer);
    println!("Difference between most and least common element after 40 steps is {}", most_common_count - least_common_count);
}

fn part1(start_polymer: &str, reaction_map: &HashMap<(char, char), char>) {
    let mut polymer = start_polymer.to_string();
    for _ in 0..10 {
        polymer = do_step(polymer, reaction_map);
    }

    let (most_common_count, least_common_count) = find_most_and_least_common_char_count_in_string(polymer);
    println!("Difference between most and least common element after 10 steps is {}", most_common_count - least_common_count);
}

fn main() {
    let start = Instant::now();

    let mut input = include_str!("../input.txt").lines();
    let start_string = input.next().expect("Invalid Input").trim();
    input.next();

    let mut map: HashMap<(char, char), char> = HashMap::new();
    for line in input {
        let (pair, output) = line.split_once(" -> ").unwrap();
        let mut chars = pair.chars();
        map.insert((chars.next().unwrap(), chars.next().unwrap()), output.chars().next().unwrap());
    }
    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1(start_string, &map);
    println!("Part 1 took {:?}", start.elapsed());

    let start = Instant::now();
    part2(start_string, &map);
    println!("Part 2 took {:?}", start.elapsed());
}
