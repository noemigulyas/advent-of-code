use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut fresh_ranges: Vec<(u64, u64)> = Vec::new();
    let mut num_available_fresh = 0;
    let mut num_fresh_total = 0;

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut tokens = line.split('-');

        fresh_ranges.push((
            tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
        ));
    }

    loop {
        let mut new = fresh_ranges.clone();

        'outer: for i in 0..new.len() {
            let (start_1, end_1) = new[i];

            for j in i + 1..new.len() {
                let (start_2, end_2) = new[j];

                if start_1 <= end_2 && start_2 <= end_1 {
                    let mut new_set: HashSet<_> = new.iter().copied().collect();
                    new_set.remove(&(start_1, end_1));
                    new_set.remove(&(start_2, end_2));

                    let true_start = start_1.min(start_2);
                    let true_end = end_1.max(end_2);
                    new_set.insert((true_start, true_end));

                    new = new_set.into_iter().collect();
                    break 'outer;
                }
            }
        }

        let old_set: HashSet<_> = fresh_ranges.iter().copied().collect();
        let new_set: HashSet<_> = new.iter().copied().collect();

        if old_set == new_set {
            break;
        }

        fresh_ranges = new;
    }

    for line in lines {
        let id: u64 = line.parse().unwrap();

        for (start, end) in fresh_ranges.iter().copied() {
            if (start..=end).contains(&id) {
                num_available_fresh += 1;
                break;
            }
        }
    }

    for (start, end) in fresh_ranges {
        num_fresh_total += end - start + 1;
    }

    println!("{num_available_fresh}");
    println!("{num_fresh_total}");
}
