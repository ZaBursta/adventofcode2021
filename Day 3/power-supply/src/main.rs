use std::ops::Index;

struct BitCriteria {
    keep_zero_on_tie: bool,
    keep_most_common: bool,
}

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

fn part2(values: impl AsRef<[i32]>, binary_length: i32) {
    let oxy_generator_criteria = BitCriteria {
        keep_most_common: true,
        keep_zero_on_tie: false
    };
    let oxy_gen_rating = part2_inner(&values, binary_length, &oxy_generator_criteria);

    let co2_scrubber_rating = BitCriteria {
        keep_most_common: false,
        keep_zero_on_tie: true,
    };
    let co2_scrub_rating = part2_inner(&values, binary_length, &co2_scrubber_rating);

    println!("Oxy Rating :{}", oxy_gen_rating);
    println!("Co2 Rating :{}", co2_scrub_rating);
    println!("Life Support Rating :{}", oxy_gen_rating * co2_scrub_rating);
}

fn should_keep_zero(sum: i32, significant_bit_value: i32, array_length: usize, settings: &BitCriteria) -> bool {
    if sum == (significant_bit_value * array_length as i32/2) {
        return settings.keep_zero_on_tie;
    } else if sum > (significant_bit_value * array_length as i32/2) {
        return !settings.keep_most_common;
    } else {
        return settings.keep_most_common
    }
}

fn part2_inner(values: impl AsRef<[i32]>, binary_length: i32, bit_criteria: &BitCriteria) -> i32 {
    let mut mutable_values = values.as_ref().to_vec();
    let mut index = binary_length - 1;

    while mutable_values.len() != 1 {
        let significant_bit_value = 2_i32.pow(index as u32) as i32;
        let sum: i32 = mutable_values.iter().map(|x| x & significant_bit_value).sum();
        if should_keep_zero(sum, significant_bit_value, mutable_values.len(), bit_criteria) {
            mutable_values = mutable_values.into_iter().filter(|x| x & significant_bit_value != 0).collect();
        } else {
            mutable_values = mutable_values.into_iter().filter(|x| x & significant_bit_value == 0).collect();
        }
        index -= 1;
    }

    return *mutable_values.index(0);
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
    part2(&values, 12);
}
