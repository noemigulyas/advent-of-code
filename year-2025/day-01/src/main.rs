use {
    crate::{command::Rotation, dial::Dial},
    std::{fs::read_to_string, str::FromStr},
};

mod command;
mod dial;

fn main() {
    let mut dial = Dial::default();

    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| Rotation::from_str(l).unwrap())
        .for_each(|r| dial.rotate(r));

    println!("{}", dial.num_zeros_method_1());
    println!("{}", dial.num_zeros_method_2());
}
