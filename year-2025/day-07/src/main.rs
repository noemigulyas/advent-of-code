use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Node = (usize, usize);
type Graph = HashMap<Node, Vec<Node>>;
const END: Node = (1000000, 1000000);

fn main() {
    let matrix: Vec<Vec<char>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut graph = Graph::new();
    let mut visited = HashSet::new();
    let mut memos = HashMap::new();
    let start_x = matrix[0].len() / 2;

    discover_graph(
        &matrix,
        &mut graph,
        &mut visited,
        (start_x, 0),
        (start_x, 1),
    );

    let num_splits = graph.len() - 1;
    let num_timelines = count_paths(&mut memos, &graph, (start_x, 0), END);

    println!("{num_splits}");
    println!("{num_timelines}");
}

fn discover_graph(
    matrix: &[Vec<char>],
    graph: &mut Graph,
    visited: &mut HashSet<Node>,
    current: Node,
    start_pos: (usize, usize),
) {
    let (x, mut y) = start_pos;

    loop {
        if y == matrix.len() {
            graph.entry(current).or_default().push(END);
            return;
        }

        if matrix[y][x] == '^' {
            graph.entry(current).or_default().push((x, y));
            if !visited.contains(&(x, y)) {
                discover_graph(matrix, graph, visited, (x, y), (x - 1, y));
                discover_graph(matrix, graph, visited, (x, y), (x + 1, y));
                visited.insert((x, y));
            }
            return;
        }

        y += 1;
    }
}

fn count_paths(memos: &mut HashMap<Node, u64>, graph: &Graph, src: Node, dst: Node) -> u64 {
    if src == dst {
        return 1;
    }

    if memos.contains_key(&src) {
        return memos[&src];
    }

    let mut sum = 0;

    for x in graph[&src].iter().copied() {
        sum += count_paths(memos, graph, x, dst)
    }

    memos.insert(src, sum);
    return sum;
}
