fn find_lowest_location(lines: &Vec<&str>) -> u64 {
    let mut lines = lines.iter();
    // get all initial intervals
    let numbers = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut intervals =
        numbers
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect::<Vec<(u64, u64)>>();
    lines.next(); // skip the empty line after seeds
    // println!("{:?}", intervals);
    // each iteration handles one map from foo to bar (like from seed to soil)
    while lines.next().is_some() {
        // the next few lines are foo-to-bar mappings: foo-from, foo-to, bar-from
        let mut mappings = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mapping = line
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            // assert there are exactly three numbers
            assert_eq!(mapping.len(), 3);
            mappings.push((
                mapping[1],
                mapping[1] + mapping[2] - 1,
                mapping[0],
            ));
        }
        // sort all mappings by foo-from
        mappings.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("mappings: {:?}", mappings);
        intervals = intervals.iter().map(| (from, to)| {
            let mut from = *from;
            let to = *to;
            let mut new_intervals = vec![];
            for (foo_from, foo_to, bar_from) in mappings.clone(){
                if foo_to < from {
                    // this mapping is before the interval
                    continue;
                }
                if foo_from > to {
                    // this mapping is after the interval
                    break;
                }
                if foo_from > from {
                    // there is a gap between the last mapping and this one
                    new_intervals.push((from, foo_from - 1));
                    from = foo_from;
                }
                let actual_to = to.min(foo_to);
                if from <= actual_to {
                    // the overlap between the interval and the mapping, [from, actual_to]
                    new_intervals.push((bar_from + from - foo_from, bar_from + actual_to - foo_from));
                    from = actual_to + 1;
                }
                // it's over
                if from > to {
                    break;
                }
            }
            // the last unmapped interval
            if from <= to {
                new_intervals.push((from, to));
            }
            new_intervals
        }).flatten().collect::<Vec<(u64, u64)>>();
        // println!("{:?}", intervals)
    }
    intervals.iter().min().unwrap().0
}

#[test]
fn test_find_lowest_location() {
    let lines = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];
    assert_eq!(find_lowest_location(&lines), 46);
}

fn process_lines_in_file(filename: &str) -> u64 {
    let content = std::fs::read_to_string(filename).unwrap();
    let lines = content.lines().collect::<Vec<&str>>();
    find_lowest_location(&lines)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let ans = process_lines_in_file(filename);
    println!("{ans}");
}
