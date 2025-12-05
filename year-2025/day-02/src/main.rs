use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let trimmed = input.trim();
    let ranges = trimmed.split(',');
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for range in ranges {
        let mut ends = range.split('-');
        let start: u64 = ends.next().unwrap().parse().unwrap();
        let end: u64 = ends.next().unwrap().parse().unwrap();

        for id in start..=end {
            if is_invalid_1(id) {
                sum_1 += id;
            }

            if is_invalid_2(id) {
                sum_2 += id;
            }
        }
    }

    println!("{sum_1}");
    println!("{sum_2}");
}

fn is_invalid_1(id: u64) -> bool {
    let id = id.to_string();

    if id.len() % 2 == 1 {
        return false;
    }

    let midpoint = id.len() / 2;
    let part_1 = &id[..midpoint];
    let part_2 = &id[midpoint..];

    part_1 == part_2
}

fn is_invalid_2(id: u64) -> bool {
    let id = id.to_string();

    'pat: for pat_len in 1..=id.len() / 2 {
        if !id.len().is_multiple_of(pat_len) {
            continue;
        }

        let pat = &id[0..pat_len];

        for i in 0..id.len() / pat_len {
            let chunk = &id[i * pat_len..(i + 1) * pat_len];

            if chunk != pat {
                continue 'pat;
            }
        }

        return true;
    }

    false
}
