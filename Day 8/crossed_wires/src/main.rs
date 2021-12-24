use std::time::Instant;
use std::collections::HashSet;

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

fn find_three(display: &Display, one: &HashSet<char>) -> HashSet<char> {
    for input in display.inputs.to_owned() {
        if input.len() == 5 {
            let three_candidate = HashSet::from_iter(input.chars());
            if one.difference(&three_candidate).count() == 0 {
                return three_candidate;
            }
        }
    }

    return HashSet::new();
}

fn find_nine(display: &Display, three: &HashSet<char>) -> HashSet<char> {
    for input in display.inputs.to_owned() {
        if input.len() == 6 {
            let nine_or_zero = HashSet::from_iter(input.chars());
            if three.difference(&nine_or_zero).count() == 0 {
                return nine_or_zero;
            }
        }
    }

    return HashSet::new();
}

fn find_two_and_five(display: &Display, nine: &HashSet<char>, three: &HashSet<char>) -> (HashSet<char>, HashSet<char>) {
    let mut five = HashSet::new();
    let mut two = HashSet::new();
    for input in display.inputs.to_owned() {
        if input.len() == 5 {
            let two_three_or_five = HashSet::from_iter(input.chars());
            if three.difference(&two_three_or_five).count() == 0 {
                continue;
            }

            if two_three_or_five.difference(&nine).count() == 0 {
                five = two_three_or_five;
            } else {
                two = two_three_or_five;
            }
        }
    }

    return (two, five);
}

fn find_zero_and_six(display: &Display, five: &HashSet<char>, nine: &HashSet<char>) -> (HashSet<char>, HashSet<char>) {
    let mut zero = HashSet::new();
    let mut six = HashSet::new();
    for input in display.inputs.to_owned() {
        if input.len() == 6 {
            let zero_six_or_nine = HashSet::from_iter(input.chars());
            if nine.difference(&zero_six_or_nine).count() == 0 {
                continue;
            }

            if five.difference(&zero_six_or_nine).count() == 0 {
                six = zero_six_or_nine;
            } else {
                zero = zero_six_or_nine;
            }
        }
    }

    return (zero, six);
}

fn find_sum_from_display(display: &Display) -> i32 {
    let mut zero: HashSet<char> = HashSet::new();
    let mut one: HashSet<char> = HashSet::new();
    let mut two: HashSet<char> = HashSet::new();
    let mut three: HashSet<char> = HashSet::new();
    let mut four: HashSet<char> = HashSet::new();
    let mut five: HashSet<char> = HashSet::new();
    let mut six: HashSet<char> = HashSet::new();
    let mut seven: HashSet<char> = HashSet::new();
    let mut eight: HashSet<char> = HashSet::new();
    let mut nine: HashSet<char> = HashSet::new();

    for input in display.inputs.to_owned() {
        match input.len() {
            2 => if one.len() == 0 { one = HashSet::from_iter(input.chars()) },
            3 => if seven.len() == 0 { seven = HashSet::from_iter(input.chars()) },
            4 => if four.len() == 0 { four = HashSet::from_iter(input.chars()) },
            7 => if eight.len() == 0 { eight = HashSet::from_iter(input.chars()) },
            _ => {}
        }
    }


    three = find_three(display, &one);
    nine = find_nine(display, &three);

    {
        let two_five = find_two_and_five(display, &nine, &three);
        two = two_five.0;
        five = two_five.1;
    }

    {
        let zero_six = find_zero_and_six(display, &five, &nine);
        zero = zero_six.0;
        six = zero_six.1;
    }

    let mut sum = 0;
    for (i, output) in display.outputs.iter().enumerate() {
        let mut val: i32 = match output.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            6 => -3,
            7 => 8,
            _ => -1,
        };

        if val == -3 { // 0, 9 or 6
            let chars = HashSet::from_iter(output.chars());
            if chars.difference(&zero).count() == 0 {
                val = 0;
            } else if chars.difference(&six).count() == 0 {
                val = 6;
            } else {
                val = 9;
            }
        } else if val == -1 { // 2, 3 5
            let chars = HashSet::from_iter(output.chars());
            if chars.difference(&two).count() == 0 {
                val = 2;
            } else if chars.difference(&three).count() == 0 {
                val = 3;
            } else {
                val = 5;
            }
        }

        sum += val * 10_i32.pow(3 - i as u32);
    }

    return sum;
}

fn part2(displays: &Vec<Display>) {
    let mut sum = 0;
    for display in displays {
        sum += find_sum_from_display(display);
    }
    println!("Total sum of all output values is {}", sum);
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
    part2(&displays);
    println!("Part 2 took {:?}", start.elapsed());
}
