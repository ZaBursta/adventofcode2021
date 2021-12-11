use std::time::Instant;

fn is_open_bracket(char: char) -> bool {
    return match char {
        '{' | '[' | '(' | '<' => true,
        '}' | ']' | ')' | '>' => false,
        _ => panic!("Not a valid character!")
    }
}

fn corrupt_score_from_char(char: char) -> i32 {
    return match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Not a scoring character")
    }
}

fn incomplete_score_from_char(char: char) -> u128 {
    return match char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
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
                    score += corrupt_score_from_char(char);
                    bracket_stack = vec![];
                    break;
                }
            }
        }
    }

    println!("Total score for corrupt lines: {}", score);
}

fn part2(lines: &Vec<&str>) {
    let mut bracket_stack = vec![];
    let mut scores = vec![];

    for line in lines {
        for char in line.chars() {
            if is_open_bracket(char) {
                bracket_stack.push(char);
            } else {
                let open_bracket = bracket_stack.pop().expect("Invalid Input. Closed Bracket without Matching Open Bracket.");
                if closed_variant_of_open_bracket(open_bracket) != char {
                    bracket_stack = vec![];
                    break;
                }
            }
        }

        let mut score: u128 = 0;
        while let Some(bracket) = bracket_stack.pop() {
            score = score * 5 + incomplete_score_from_char(closed_variant_of_open_bracket(bracket));
            if score > std::u64::MAX as u128 {
                score = std::u128::MAX;
                break;
            }
        }

        if score != 0 {
            scores.push(score);
        }
    }

    scores.sort();
    println!("Middle score out is {})", scores.get(scores.len()/2).expect(""));
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

    let start = Instant::now();
    part2(&lines);
    println!("Part 2 took {:?}", start.elapsed());
}
