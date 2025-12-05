use std::fs::read_to_string;

#[derive(Clone)]
pub struct Grid {
    cells: Vec<char>,
    width: isize,
    height: isize,
}

impl Grid {
    pub fn load() -> Self {
        let input = read_to_string("input.txt").unwrap();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut cells = vec!['.'; width * height];

        for (y, line) in input.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                cells[y * width + x] = cell;
            }
        }

        Self {
            cells,
            width: width as isize,
            height: height as isize,
        }
    }

    pub fn find_and_remove(&self) -> (u32, Self) {
        let mut sum = 0;
        let mut new_grid = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                if !self.is_paper_roll(x, y) {
                    continue;
                }

                let mut num = 0;

                for y_1 in y - 1..=y + 1 {
                    for x_1 in x - 1..=x + 1 {
                        if x_1 == x && y_1 == y {
                            continue;
                        }

                        if self.is_paper_roll(x_1, y_1) {
                            num += 1;
                        }
                    }
                }

                if num < 4 {
                    sum += 1;
                    new_grid.cells[(y * self.width + x) as usize] = '.';
                }
            }
        }

        (sum, new_grid)
    }

    fn is_paper_roll(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }

        let cell = self.cells[(y * self.width + x) as usize];
        cell == '@'
    }
}
