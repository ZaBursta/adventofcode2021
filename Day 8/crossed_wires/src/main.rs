use std::time::Instant;

struct Display {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

fn part1(displays: &Vec<Display>) {
    let mut count_of_1_4_7_8 = 0;

    for display in displays {
        for output in display.outputs.to_owned() {
            match output.len() {
                2 => count_of_1_4_7_8 += 1,
                3 => count_of_1_4_7_8 += 1,
                4 => count_of_1_4_7_8 += 1,
                7 => count_of_1_4_7_8 += 1,
                _ => {}
            }
        }
    }

    println!("There are {} 1s, 4s, 7s and 8s in the output numbers", count_of_1_4_7_8);
}
fn main() {
    let start = Instant::now();

    let displays: Vec<Display> = include_str!("../input.txt")
        .lines()
        .map(|x| {
            let (input, output) = x.split_once('|').expect("Invalid Input");
            Display {
                inputs: input.trim().split_whitespace().map(|x| x.to_string()).collect(),
                outputs: output.trim().split_whitespace().map(|x| x.to_string()).collect(),
            }
        })
        .collect();
    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1(&displays);
    println!("Part 1 took {:?}", start.elapsed());

    let start = Instant::now();
    println!("Part 2 took {:?}", start.elapsed());
}
