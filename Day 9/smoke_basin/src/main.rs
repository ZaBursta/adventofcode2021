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
}
