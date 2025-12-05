use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let total_joltage_1: u64 = input.trim().lines().map(|l| find_max(l, 1)).sum();
    println!("{total_joltage_1}");

    let total_joltage_2: u64 = input.trim().lines().map(|l| find_max(l, 11)).sum();
    println!("{total_joltage_2}");
}

fn find_max(line: &str, power: u32) -> u64 {
    let digits: Vec<u64> = line.chars().map(|c| c as u64 - '0' as u64).collect();
    let mut max: u64 = 0;
    find_max_recurse(&digits, &mut max, power, 0, 0);
    max
}

fn find_max_recurse(digits: &[u64], max: &mut u64, power: u32, start: usize, value: u64) {
    for i in start..digits.len() {
        let digit = digits[i];

        if value + (digit + 1) * 10u64.pow(power) - 1 < *max {
            continue;
        }

        let value = value + digit * 10u64.pow(power);

        if power == 1 {
            for j in i + 1..digits.len() {
                let digit = digits[j];
                let value = value + digit;

                if value > *max {
                    *max = value;
                }
            }

            continue;
        }

        find_max_recurse(digits, max, power - 1, i + 1, value);
    }
}
