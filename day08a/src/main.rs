use std::collections::HashMap;

const START: &str = "AAA";
const END: &str = "ZZZ";

fn count_steps(lines: Vec<&str>) -> u64 {
    let mut iter = lines.into_iter();
    // get the sequence of instructions from the first line
    let instructions = iter.next().unwrap().chars().collect::<Vec<char>>();
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
    // following the instructions, find the minimum number of steps from START to END
    1 + instructions
        .into_iter()
        .cycle()
        .scan(START, |node, instruction| {
            let (left, right) = graph.get(node).unwrap();
            *node = match instruction {
                'L' => left,
                'R' => right,
                _ => panic!("invalid instruction"),
            };
            Some(*node)
        })
        .take_while(|node| *node != END)
        .count() as u64
}

#[test]
fn test_count_steps() {
    assert_eq!(
        count_steps(vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]),
        2
    );

    assert_eq!(
        count_steps(vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ]),
        6
    );
}

fn process_file(filename: &str) -> u64 {
    let content = std::fs::read_to_string(filename).unwrap();
    let lines = content.lines().collect::<Vec<&str>>();
    count_steps(lines)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let ans = process_file(filename);
    println!("{ans}");
}
