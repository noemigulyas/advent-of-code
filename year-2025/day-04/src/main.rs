mod grid;

use crate::grid::Grid;

fn main() {
    let mut grid = Grid::load();
    let (removable_rolls_1, _) = grid.find_and_remove();
    println!("{removable_rolls_1}");

    let mut total_rolls = 0;

    loop {
        let (rolls, next) = grid.find_and_remove();

        if rolls == 0 {
            break;
        }

        total_rolls += rolls;
        grid = next;
    }

    println!("{total_rolls}")
}
