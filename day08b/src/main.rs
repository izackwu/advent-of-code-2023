use num::integer::lcm;
use std::collections::{HashMap, HashSet};

// Starting at `start`, follow the instructions to reach two nodes (or the same node twice) ending with 'Z'.
fn reach_z_twice(
    graph: &HashMap<&str, (&str, &str)>,
    instructions: &str,
    start: &str,
) -> (u64, u64) {
    // following the instructions, find the minimum number of steps from START to END
    let steps = instructions
        .chars()
        .into_iter()
        .cycle()
        .scan((start, 0), |(node, steps), instruction| {
            let (left, right) = graph.get(node).unwrap();
            *node = match instruction {
                'L' => left,
                'R' => right,
                _ => panic!("invalid instruction"),
            };
            *steps += 1;
            Some((*node, *steps))
        })
        .filter_map(|(node, steps)| {
            if node.ends_with('Z') {
                Some(steps)
            } else {
                None
            }
        })
        .take(2)
        .collect::<Vec<_>>();
    (steps[0], steps[1])
}

fn count_steps_as_a_ghost(lines: Vec<&str>) -> u64 {
    let mut iter = lines.into_iter();
    // get the sequence of instructions from the first line
    let instructions = iter.next().unwrap();
    // skip an empty line
    iter.next();
    // build the node graph: each node has exactly two children
    let mut graph = HashMap::new();
    iter.for_each(|line| {
        // each line is in the format of "FOO = (BAR, BAZ)"
        let from = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        graph.insert(from, (left, right));
    });
    let nodes = graph
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<HashSet<_>>();
    nodes
        .iter()
        .map(|node| {
            let (first, second) = reach_z_twice(&graph, &instructions, node);
            // This is tricky: the input data guarantees that for each starting node, for reaching the first Z node,
            // it takes the last step in the instructions and returns to the starting point.
            assert!(first * 2 == second);
            first
        })
        .fold(1, |acc, x| lcm(acc, x))
}

#[test]
fn test_count_steps() {
    assert_eq!(
        count_steps_as_a_ghost(vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]),
        6
    );
}

fn process_file(filename: &str) -> u64 {
    let content = std::fs::read_to_string(filename).unwrap();
    let lines = content.lines().collect::<Vec<&str>>();
    count_steps_as_a_ghost(lines)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let ans = process_file(filename);
    println!("{ans}");
}
