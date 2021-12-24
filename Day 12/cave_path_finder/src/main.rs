use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn search_cave_connections(cave_connections: &HashMap<String, Vec<String>>, seen_caves: &mut HashSet<String>, current_cave: String) -> Vec<String> {
    if current_cave == "end" {
        return vec!["".to_string()];
    }

    if current_cave.to_lowercase() == current_cave {
        seen_caves.insert(current_cave.to_owned());
    }

    let mut to_return = vec![];
    for connection in cave_connections.get(&current_cave).expect("Missing Key!") {
        if !seen_caves.contains(connection) {
            to_return.append(&mut search_cave_connections(cave_connections, seen_caves, connection.to_owned())
                .iter()
                .map(|cave_path| format!("{},{}", connection, cave_path))
                .collect());
        }
    }

    seen_caves.remove(&current_cave);
    return to_return;
}

fn part1(cave_connections: &HashMap<String, Vec<String>>) {
    let cave_paths = search_cave_connections(cave_connections, &mut HashSet::new(), "start".to_string());
    println!("There are {} possible connections", cave_paths.len());
}
fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");
    let mut cave_connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (cave_one, cave_two) = line.split_once('-').expect("Invalid Input");
        cave_connections.entry(cave_one.to_string()).or_insert(vec![]).push(cave_two.to_string());
        cave_connections.entry(cave_two.to_string()).or_insert(vec![]).push(cave_one.to_string());
    }

    println!("Prep took {:?}", start.elapsed());

    println!("{:?}", cave_connections);

    let start = Instant::now();
    part1(&cave_connections);
    println!("Part 1 took {:?}", start.elapsed());

}
