fn get_low_points(cavern: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut to_return = vec![];
    for (row_index, row) in cavern.iter().enumerate() {
        for (column_index, depth) in row.iter().enumerate() {
            if row_index != 0 && cavern[row_index - 1][column_index] <= *depth {
                continue;
            }

            if row_index != cavern.len() - 1 && cavern[row_index + 1][column_index] <= *depth {
                continue;
            }

            if column_index != 0 && cavern[row_index][column_index - 1] <= *depth {
                continue;
            }

            if column_index != cavern[row_index].len() - 1 && cavern[row_index][column_index + 1] <= *depth {
                continue;
            }

            to_return.push((row_index, column_index));
        }
     }

     return to_return;
}

fn part1(cavern: &Vec<Vec<u32>>) {
    let low_points = get_low_points(cavern);
    let mut risk_level_sum = 0;
    for (row_index, column_index) in low_points {
        risk_level_sum += 1 + cavern[row_index][column_index];
    }

    println!("Risk Level Sum is {}", risk_level_sum);
}

fn depth_first_search(cavern: &Vec<Vec<u32>>, depth: u32, row_index: usize, column_index: usize, visited: &mut Vec<Vec<bool>>) -> u32 {
    if visited[row_index][column_index] ||
       cavern[row_index][column_index] == 9 ||
       cavern[row_index][column_index] < depth
    {
        return 0;
    }

    visited[row_index][column_index] = true;

    let mut basin_size = 1;
    let depth = cavern[row_index][column_index];
    if row_index != 0 {
        basin_size += depth_first_search(cavern, depth, row_index - 1, column_index, visited);
    }

    if row_index != cavern.len() - 1 {
        basin_size += depth_first_search(cavern, depth, row_index + 1, column_index, visited);
    }

    if column_index != 0 {
        basin_size += depth_first_search(cavern, depth, row_index, column_index - 1, visited);
    }

    if column_index != cavern[row_index].len() - 1 {
        basin_size += depth_first_search(cavern, depth, row_index, column_index + 1, visited);
    }

    return basin_size
}

fn part2(cavern: &Vec<Vec<u32>>) {
    let low_points = get_low_points(cavern);
    let mut top_basins: [u32; 3] = [0; 3];
    let mut visited_squares: Vec<Vec<bool>> = vec![];

    let mut row_index = 0;
    for row in cavern {
        visited_squares.push(vec![]);
        for _column in row {
            visited_squares[row_index].push(false);
        }
        row_index += 1;
    }

    for (row_index, column_index) in low_points {
        let basin_size = depth_first_search(cavern, cavern[row_index][column_index], row_index, column_index, &mut visited_squares);

        if basin_size > top_basins[0] {
            top_basins[2] = top_basins[1];
            top_basins[1] = top_basins[0];
            top_basins[0] = basin_size;
        } else if basin_size > top_basins[1] {
            top_basins[2] = top_basins[1];
            top_basins[1] = basin_size;
        } else if basin_size > top_basins[2] {
            top_basins[2] = basin_size;
        }
    }

    println!("3 top sized basins are {}, {}, {} with a multiple of {}", top_basins[0], top_basins[1], top_basins[2], top_basins[0] * top_basins[1] *top_basins[2]);
}

fn main() {
    let mut cavern: Vec<Vec<u32>> = Default::default();

    let input = include_str!("../input.txt");
    for (row_index, line) in input.lines().enumerate() {
        cavern.push(vec![]);
        for char in line.chars() {
            cavern[row_index].push(char.to_digit(10).expect("Invalid Input"));
        }
    }

    part1(&cavern);
    part2(&cavern);
}
