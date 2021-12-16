use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn point_from_str(str: &str) -> Point {
    let (x_str, y_str) = str
     .trim()
     .split_once(",")
     .unwrap();

     return Point {
         x: x_str.parse().unwrap(),
         y: y_str.parse().unwrap(),
    }
}

fn points_between_points(point_one: Point, point_two: Point) -> Vec<Point> {
    let mut current_point = point_one;
    let end_point = point_two;
    let mut to_return = vec![current_point];

    while current_point != end_point {
        if current_point.x < end_point.x {
            current_point.x += 1;
        } else if current_point.x > end_point.x {
            current_point.x -= 1;
        }

        if current_point.y < end_point.y {
            current_point.y += 1;
        } else if current_point.y > end_point.y {
            current_point.y -= 1;
        }

        to_return.push(current_point);
    }

    return to_return;
}

fn print_dangerous_vent_count(vents: &Vec<(Point, Point)>, include_diagonal_lines: bool) {
    let mut ocean_floor: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    for vent in vents {
        if !include_diagonal_lines && vent.0.x != vent.1.x && vent.0.y != vent.1.y {
            continue;
        }

        for location in points_between_points(vent.0, vent.1) {
            *ocean_floor
            .entry(location.x)
            .or_insert(HashMap::new())
            .entry(location.y)
            .or_insert(0) += 1;
        }
    }

    let mut count_danger_areas = 0;
    for column in ocean_floor.values() {
        for overlapping_lines in column.values() {
            if *overlapping_lines > 1 {
                count_danger_areas += 1;
            }
        }
    }

    println!("There are {} points where at least 2 lines overlap", count_danger_areas);
}

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let vents: Vec<(Point, Point)> = input
                                      .lines()
                                      .map(|x| {
                                          let (point_0, point_1) = x.split_once(" -> ").unwrap();
                                          return (point_from_str(point_0), point_from_str(point_1));
                                      })
                                      .collect();
    println!("Input processing took {:?}", start.elapsed());

    let start = Instant::now();
    print_dangerous_vent_count(&vents,false);
    println!("Part 1 took {:?}", start.elapsed());

    let start = Instant::now();
    print_dangerous_vent_count(&vents,true);
    println!("Part 2 took {:?}", start.elapsed());
}
