fn part1(values: impl AsRef<[i32]>, binary_length: i32) {
    let values = values.as_ref();
    let mut output = 0;

    for i in 0..binary_length {
        let sig_fig_bit = 2_i32.pow((binary_length - i - 1).try_into().unwrap());
        let sum: i32 = values.iter().map(|x| x & sig_fig_bit).sum();
        if sum > (sig_fig_bit * values.len() as i32/2) {
            output += sig_fig_bit;
        }
    }
    let max_value = 2_i32.pow((binary_length).try_into().unwrap()) - 1;
    println!("Gamma Rate: {} Epsilon Rate: {}", output, !output & max_value);
    println!("Power Consupmtion: {}", output * (!output & max_value));
}

 fn main() {
    let input = include_str!("../input.txt");
    let values: Vec<i32> = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.is_empty() {
                let mut value = 0;
                for (i, c) in line.chars().enumerate() {
                    if c == '1' {
                        value += 2_i32.pow((line.len() - i - 1).try_into().unwrap());
                    }
                }
                // println!("{} is {}", line, value);
                Some(value)
            } else {
                None
            }
        })
        .collect();
    part1(&values, 12);
}
