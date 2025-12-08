use {
    glam::DVec3,
    std::{collections::HashSet, fs::read_to_string},
};

fn main() {
    solve_part_1();
    solve_part_2();
}

fn solve_part_1() {
    let mut circuits = load_junc_boxes("input.txt");
    let mut direct_connections = HashSet::new();

    for i in 0..1000 {
        connect_shortest(&mut circuits, &mut direct_connections);
        println!("{i}/1000");
    }

    circuits.sort_by_key(|c| -(c.len() as isize));

    let mut prod = 1;

    for circuit in circuits.iter().take(3) {
        prod *= circuit.len();
    }

    println!("{prod}");
}

fn solve_part_2() {
    let mut circuits = load_junc_boxes("input.txt");
    let mut direct_connections = HashSet::new();

    while circuits.len() > 1 {
        let result = connect_shortest(&mut circuits, &mut direct_connections);
        println!("{} -> 1", circuits.len());

        if let Some((a, b)) = result
            && circuits.len() == 1
        {
            println!("{}", a.x * b.x);
        }
    }
}

fn load_junc_boxes(path: &str) -> Vec<HashSet<JuncBox>> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let mut tokens = l.split(',');
            let x: i32 = tokens.next().unwrap().parse().unwrap();
            let y: i32 = tokens.next().unwrap().parse().unwrap();
            let z: i32 = tokens.next().unwrap().parse().unwrap();
            [JuncBox { x, y, z }].into_iter().collect()
        })
        .collect()
}

fn connect_shortest(
    circuits: &mut Vec<HashSet<JuncBox>>,
    direct_connections: &mut HashSet<(JuncBox, JuncBox)>,
) -> Option<(JuncBox, JuncBox)> {
    let mut closest_pair = None;

    for i in 0..circuits.len() {
        for j in 0..circuits.len() {
            for a in circuits[i].iter().copied() {
                for b in circuits[j].iter().copied() {
                    if a == b {
                        continue;
                    }

                    if direct_connections.contains(&(a, b)) || direct_connections.contains(&(b, a))
                    {
                        continue;
                    }

                    match &mut closest_pair {
                        Some((_, _, min_dist_sqr)) => {
                            let dist_sqr = a.distance_squared(b);
                            if dist_sqr < *min_dist_sqr {
                                closest_pair = Some(((i, a), (j, b), dist_sqr));
                            }
                        }
                        None => {
                            closest_pair = Some(((i, a), (j, b), a.distance_squared(b)));
                        }
                    }
                }
            }
        }
    }

    let ((i, a), (j, b), _) = closest_pair.unwrap();
    direct_connections.insert((a, b));

    if i == j {
        return None;
    }

    let mut merged_circuits: HashSet<JuncBox> = HashSet::new();
    merged_circuits.extend(&circuits[i]);
    merged_circuits.extend(&circuits[j]);

    circuits.remove(i.max(j));
    circuits.remove(i.min(j));
    circuits.push(merged_circuits);

    Some((a, b))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JuncBox {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl JuncBox {
    pub fn distance_squared(&self, b: Self) -> f64 {
        DVec3::new(self.x as f64, self.y as f64, self.z as f64)
            .distance_squared(DVec3::new(b.x as f64, b.y as f64, b.z as f64))
    }
}
