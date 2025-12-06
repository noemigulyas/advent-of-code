use std::fs::read_to_string;

pub fn solve() {
    let matrix: Vec<Vec<char>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let width = matrix[0].len();
    let height = matrix.len();

    let mut nums = Vec::new();
    let mut total = 0;

    'outer: for x in (0..width).rev() {
        let mut digits = Vec::new();
        let mut y = 0;

        while matrix[y][x] == ' ' {
            y += 1;

            if y == height {
                nums.clear();
                continue 'outer;
            }
        }

        while y < height {
            let c = matrix[y][x];

            match c {
                '0'..='9' => {
                    let digit = c as u64 - '0' as u64;
                    digits.push(digit);
                }

                ' ' | '+' | '*' => {
                    if !digits.is_empty() {
                        let mut num = 0;
                        let mut exp = 1;

                        for digit in digits.iter().copied().rev() {
                            num += digit * exp;
                            exp *= 10;
                        }

                        nums.push(num);
                        digits.clear();
                    }

                    if c == '+' {
                        let subtotal: u64 = nums.iter().sum();
                        total += subtotal;
                    }

                    if c == '*' {
                        let subtotal: u64 = nums.iter().product();
                        total += subtotal;
                    }
                }

                _ => panic!(),
            }

            y += 1;
        }
    }

    println!("{total}")
}
