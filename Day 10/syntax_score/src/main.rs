use std::time::Instant;

fn is_open_bracket(char: char) -> bool {
    return match char {
        '{' | '[' | '(' | '<' => true,
        '}' | ']' | ')' | '>' => false,
        _ => panic!("Not a valid character!")
    }
}

fn score_from_char(char: char) -> i32 {
    return match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Not a scoring character")
    }
}

fn closed_variant_of_open_bracket(char: char) -> char {
    return match char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Not a scoring character")
    }
}

fn part1(lines: &Vec<&str>) {
    let mut bracket_stack = vec![];
    let mut score = 0;
    for line in lines {
        for char in line.chars() {
            if is_open_bracket(char) {
                bracket_stack.push(char);
            } else {
                let open_bracket = bracket_stack.pop().expect("Invalid Input. Closed Bracket without Matching Open Bracket.");
                if closed_variant_of_open_bracket(open_bracket) != char {
                    score += score_from_char(char);
                    bracket_stack = vec![];
                    break;
                }
            }
        }
    }

    println!("Total score: {}", score);
}

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");
    let lines: Vec<&str> = input
    .lines()
    .map(|x| x.trim())
    .collect();
    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1(&lines);
    println!("Part 1 took {:?}", start.elapsed());
}
