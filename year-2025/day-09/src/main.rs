use std::fs::read_to_string;

type Element = u64;
type Point = (Element, Element);

fn main() {
    let red_tiles: Vec<Point> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(parse_point)
        .collect();

    solve_part_1(&red_tiles);
    solve_part_2(&red_tiles);
}

fn parse_point(l: &str) -> Point {
    let mut t = l.split(',');
    (
        t.next().unwrap().parse().unwrap(),
        t.next().unwrap().parse().unwrap(),
    )
}

fn solve_part_1(red_tiles: &[Point]) {
    let mut max_area = 0;

    for corner_1 in red_tiles.iter().copied() {
        for corner_2 in red_tiles.iter().copied() {
            let rect_min = (corner_1.0.min(corner_2.0), corner_1.1.min(corner_2.1));
            let rect_max = (corner_1.0.max(corner_2.0), corner_1.1.max(corner_2.1));
            let area = rect_area(rect_min, rect_max);

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{max_area}");
}

fn solve_part_2(red_tiles: &[Point]) {
    let mut max_area = 0;

    for corner_1 in red_tiles.iter().copied() {
        'inner: for corner_2 in red_tiles.iter().copied() {
            let rect_min = (corner_1.0.min(corner_2.0), corner_1.1.min(corner_2.1));
            let rect_max = (corner_1.0.max(corner_2.0), corner_1.1.max(corner_2.1));

            for i in 0..red_tiles.len() {
                let j = (i + 1) % red_tiles.len();
                let line_start = red_tiles[i];
                let line_end = red_tiles[j];

                if line_goes_inside_rect(line_start, line_end, rect_min, rect_max) {
                    continue 'inner;
                }
            }

            let area = rect_area(rect_min, rect_max);

            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{max_area}");
}

fn line_goes_inside_rect(
    line_start: Point,
    line_end: Point,
    rect_min: Point,
    rect_max: Point,
) -> bool {
    if line_start.0 == line_end.0 {
        // Vertical line

        if (line_start.1 <= rect_min.1 && line_end.1 <= rect_min.1)
            || (line_start.1 >= rect_max.1 && line_end.1 >= rect_max.1)
        {
            return false;
        }

        line_start.0 > rect_min.0 && line_start.0 < rect_max.0
    } else {
        // Horizontal line

        if (line_start.0 <= rect_min.0 && line_end.0 <= rect_min.0)
            || (line_start.0 >= rect_max.0 && line_end.0 >= rect_max.0)
        {
            return false;
        }

        line_start.1 > rect_min.1 && line_start.1 < rect_max.1
    }
}

fn rect_area(rect_min: Point, rect_max: Point) -> u64 {
    let width = rect_max.0 - rect_min.0 + 1;
    let height = rect_max.1 - rect_min.1 + 1;
    width * height
}
