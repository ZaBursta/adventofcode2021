enum Direction {
    Up,
    Down,
    Forward
}
struct Movement {
    direction: Direction,
    distance: i32,
}

fn part1(movements: impl AsRef<[Movement]>) {
    let mut depth = 0;
    let mut dist = 0;

    for movement in movements.as_ref() {
        if matches!(movement.direction, Direction::Forward) {
            dist += movement.distance;
        } else {
            depth += movement.distance * if matches!(movement.direction, Direction::Up) { -1 } else { 1 };
        }
    }

    println!("Depth of {}, distance of {}, product is {}", depth, dist, depth*dist);
}

fn part2(movements: impl AsRef<[Movement]>) {
    let mut depth = 0;
    let mut dist = 0;
    let mut aim = 0;

    for movement in movements.as_ref() {
        if matches!(movement.direction, Direction::Forward) {
            dist += movement.distance;
            depth += movement.distance * aim;
        } else {
            aim += movement.distance * if matches!(movement.direction, Direction::Up) { -1 } else { 1 };
        }
    }

    println!("Using Aim: Depth of {}, distance of {}, product is {}", depth, dist, depth*dist);
}
 fn main() {

    let input = include_str!("../input.txt");

    let values: Vec<Movement> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if !x.is_empty() {
                let (direction, distance) = x.split_once(' ').expect("");
                Some(Movement{
                    direction: match direction {
                        "down" => Direction::Down,
                        "up" => Direction::Up,
                        "forward" => Direction::Forward,
                        _ => Direction::Forward,
                    },
                    distance:distance.parse().unwrap()
                })
            } else {
                None
            }
        })
        .collect();

    part1(&values);
    part2(&values);
}
