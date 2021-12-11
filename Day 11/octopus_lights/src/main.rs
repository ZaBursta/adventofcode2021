use std::time::Instant;

fn do_flash(octopuses: &mut [[u32; 12]; 12], flashed_grid: &mut [[bool; 12]; 12], row_index: usize, column_index: usize) -> u32 {
    if flashed_grid[row_index][column_index] {
        return 0;
    }

    flashed_grid[row_index][column_index] = true;
    let mut total_flashes = 1;

    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index - 1, column_index - 1);
    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index - 1, column_index + 1);
    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index - 1, column_index);
    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index + 1, column_index - 1);
    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index + 1, column_index + 1);
    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index + 1, column_index);
    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index, column_index - 1);
    total_flashes += increment_energy_level(octopuses, flashed_grid, row_index, column_index + 1);

    return total_flashes;
}

fn increment_energy_level(octopuses: &mut [[u32; 12]; 12], flashed_grid: &mut [[bool; 12]; 12], row_index: usize, column_index: usize) -> u32 {
    if row_index == 0 ||
       column_index == 0 ||
       row_index == octopuses.len() - 1 ||
       column_index == octopuses[row_index].len() - 1 {
        return 0;
    }

    octopuses[row_index][column_index] += 1;
    if octopuses[row_index][column_index] > 9 {
        return do_flash(octopuses, flashed_grid, row_index, column_index);
    } else {
        return 0;
    }
}

fn do_step(octopuses: &mut [[u32; 12]; 12]) -> u32{
    let mut total_flashes = 0;
    let mut flashed_grid: [[bool; 12]; 12] = [[false; 12]; 12];

    // Add Energy
    for row_index in 1..octopuses.len()-1 {
        for column_index in 1..octopuses[row_index].len()-1 {
            total_flashes += increment_energy_level(octopuses, &mut flashed_grid, row_index, column_index);
        }
    }

    // Reset to 0
    for row_index in 1..octopuses.len()-1 {
        for column_index in 1..octopuses[row_index].len()-1 {
            if flashed_grid[row_index][column_index] {
                octopuses[row_index][column_index] = 0;
            }
        }
    }

    return total_flashes;
}

fn part1(octopuses: &mut [[u32; 12]; 12]) {
    let mut total_flashes = 0;
    for _i in 0..100 {
        total_flashes += do_step(octopuses);
    }

    println!("Total Flashes = {}", total_flashes);
}

fn part2(octopuses: &mut [[u32; 12]; 12]) {
    let mut total_flashes = 0;
    let mut step = 0;
    while total_flashes != 100 {
        total_flashes = do_step(octopuses);
        step += 1;
    }
    println!("Synced after {} steps", step);
}

fn main() {
    let start = Instant::now();

    let input = include_str!("../input.txt");
    let mut octopuses: Vec<[u32; 12]> = input
         .lines()
         .map(|x| {
            let mut row: [u32; 12] = [0; 12];
            for (i, char) in x.chars().enumerate() {
                row[i + 1] = char.to_digit(10).expect("");
            }
            row
        })
        .collect();
    octopuses.push([0; 12]);
    octopuses.insert(0, [0; 12]);
    let mut octopuses: [[u32; 12]; 12] = octopuses.try_into().unwrap();
    let mut octopuses_part_2: [[u32; 12]; 12] = octopuses.to_owned();

    println!("Prep took {:?}", start.elapsed());

    let start = Instant::now();
    part1(&mut octopuses);
    println!("Part 1 took {:?}", start.elapsed());

    let start = Instant::now();
    part2(&mut octopuses_part_2);
    println!("Part 2 took {:?}", start.elapsed());
}
