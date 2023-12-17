use std::collections::{HashMap, HashSet};

const MIN_STEPS_ON_ONE_DIRECTION: u32 = 4;
const MAX_STEPS_ON_ONE_DIRECTION: u32 = 10;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Node {
    i: i32,
    j: i32,
    dir: Direction,
    steps_on_this_direction: u32,
}

fn forward(node: &Node) -> Node {
    let (i, j) = match node.dir {
        Direction::Up => (node.i - 1, node.j),
        Direction::Down => (node.i + 1, node.j),
        Direction::Left => (node.i, node.j - 1),
        Direction::Right => (node.i, node.j + 1),
    };
    Node {
        i,
        j,
        dir: node.dir,
        steps_on_this_direction: node.steps_on_this_direction + 1,
    }
}

fn left(node: &Node) -> Node {
    let dir = match node.dir {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    };
    forward(&Node {
        i: node.i,
        j: node.j,
        dir,
        steps_on_this_direction: 0,
    })
}

fn right(node: &Node) -> Node {
    let dir = match node.dir {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    };
    forward(&Node {
        i: node.i,
        j: node.j,
        dir,
        steps_on_this_direction: 0,
    })
}

fn minimize_heat_loss(lines: Vec<Vec<u32>>) -> u32 {
    // build the graph: each node is (i, j, dir, steps_on_this_direction)
    let m = lines.len() as i32;
    let n = lines[0].len() as i32;
    let next_nodes = |node: &Node| {
        let nodes = if node.steps_on_this_direction < MIN_STEPS_ON_ONE_DIRECTION {
            // we must go forward
            vec![forward(node)]
        } else {
            // we can go forward, left or right
            vec![forward(node), left(node), right(node)]
        };
        // then filter out nodes that are out of bound or on the same direction for too long
        nodes
            .into_iter()
            .filter_map(|node| {
                let (i, j) = (node.i, node.j);
                if i >= 0
                    && i < m
                    && j >= 0
                    && j < n
                    && node.steps_on_this_direction <= MAX_STEPS_ON_ONE_DIRECTION
                {
                    Some((node, lines[i as usize][j as usize]))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };
    let mut graph = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            for dir in DIRECTIONS {
                for steps_on_this_direction in 1..=(MAX_STEPS_ON_ONE_DIRECTION + 1) {
                    let node = Node {
                        i,
                        j,
                        dir,
                        steps_on_this_direction,
                    };
                    let next_nodes = next_nodes(&node);
                    graph.insert(node, next_nodes);
                }
            }
        }
    }
    // define a virtual node that connects to (0, 0, *, 1) nodes
    let start = Node {
        i: 0,
        j: 0,
        dir: Direction::Down,
        steps_on_this_direction: 0,
    };
    graph.insert(
        start.clone(),
        DIRECTIONS
            .iter()
            .map(|dir| {
                (
                    Node {
                        i: 0,
                        j: 0,
                        dir: *dir,
                        steps_on_this_direction: 1,
                    },
                    0,
                )
            })
            .collect::<Vec<_>>(),
    );
    // Dijkstra's algorithm to find the shortest path from the virtual node to any (m - 1, n - 1, *, *) node.
    let mut dist = HashMap::new();
    // let mut prev = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = std::collections::BinaryHeap::new();
    dist.insert(start, 0);
    queue.push(std::cmp::Reverse((0, start)));
    while let Some(std::cmp::Reverse((d, node))) = queue.pop() {
        if visited.contains(&node) {
            continue;
        }
        if (node.i, node.j) == (m - 1, n - 1)
            && node.steps_on_this_direction >= MIN_STEPS_ON_ONE_DIRECTION
        {
            return d;
        }
        visited.insert(node);
        for (next_node, heat_loss) in graph.get(&node).unwrap() {
            let next_d = d + heat_loss;
            if !dist.contains_key(next_node) || next_d < *dist.get(next_node).unwrap() {
                dist.insert(*next_node, next_d);
                queue.push(std::cmp::Reverse((next_d, *next_node)));
            }
        }
    }
    assert!(false);
    0
}

#[test]
fn test_minimize_heat_loss() {
    assert_eq!(
        minimize_heat_loss(vec![
            vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ]),
        94
    );
    assert_eq!(
        minimize_heat_loss(vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 1],
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 1],
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 1],
            vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 1],
        ]),
        71
    )
}

fn process_file(filename: &str) -> u32 {
    let content = std::fs::read_to_string(filename).unwrap();
    let lines = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    minimize_heat_loss(lines)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let ans = process_file(filename);
    println!("{ans}");
}
