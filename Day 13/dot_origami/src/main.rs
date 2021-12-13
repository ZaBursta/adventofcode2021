use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use std::cmp;

fn new_dot_from_fold(dot: (u32, u32), fold: (bool, u32)) -> (u32, u32) {
    let (is_horizontal_fold, index) = fold;
    let (column_index, row_index) = dot;
    if is_horizontal_fold && row_index > index {
        return (column_index, 2*index - row_index);
    } else if !is_horizontal_fold && column_index > index {
        return (2*index - column_index, row_index);
    } else {
        return (column_index, row_index);
    }
}

fn part2(dots: &mut Vec<(u32, u32)>, folds: &Vec<(bool, u32)>) {
    for fold in folds.to_owned() {
        for dot_index in 0..dots.len() {
            let dot = new_dot_from_fold(dots.get(dot_index).expect("").to_owned(), fold);
            dots[dot_index] = dot;
        }
    }

    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut column_max = 0;
    let mut row_max = 0;
    for (column_index, row_index) in dots.to_owned() {
        map.entry(row_index).or_insert(HashSet::new()).insert(column_index);
        row_max = cmp::max(row_max, row_index);
        column_max = cmp::max(column_max, column_index);
    }

    // Print
    for row_index in 0..=row_max {
        for column_index in 0..=column_max {
            print!("{}", if map.entry(row_index).or_default().contains(&column_index) { "X" } else { "-" });
        }
        println!("");
    }
}

fn part1(dots: &Vec<(u32, u32)>, fold: (bool, u32)) {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for dot_index in 0..dots.len() {
        let (column_index, row_index) = new_dot_from_fold(dots.get(dot_index).expect("").to_owned(), fold);
        map.entry(column_index).or_insert(HashSet::new()).insert(row_index);
    }

    let mut dot_count = 0;
    for (_, set_of_row_indexes) in map {
        dot_count += set_of_row_indexes.len();
    }

    println!("{} dots visible after first {} fold", dot_count, if fold.0 { "horizontal" } else { "vertical" });
}

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");

    let mut dots: Vec<(u32, u32)> = vec![];
    let mut folds: Vec<(bool, u32)> = vec![];
    let mut row_max = 0;
    let mut column_max = 0;
    for line in input.lines() {
        if line.contains(',') {
            let (str1, str2) = line.split_once(',').expect("");
            let column_index = str1.parse().unwrap();
            let row_index = str2.parse().unwrap();
            dots.push((column_index, row_index));

            row_max = cmp::max(row_index, row_max);
            column_max = cmp::max(column_index, column_max);
        } else if line.contains('=') {
            let (str1, str2) = line.split_once('=').expect("");
            if str1.contains('x') {
                folds.push((false, str2.parse().unwrap()));
            } else {
                folds.push((true, str2.parse().unwrap()));
            }
        }
    }
    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1(&dots, folds[0]);
    println!("Part 1 took {:?}", start.elapsed());

    let start = Instant::now();
    part2(&mut dots, &folds);
    println!("Part 2 took {:?}", start.elapsed());
}
