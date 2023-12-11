use std::{collections::HashSet, io::BufRead};

fn check_scratchcard(line: &str) -> u32 {
    let without_card_id = line[line.find(':').unwrap() + 1..].trim();
    let mut winning_numbers_and_my_numbers = without_card_id.split('|');
    fn get_numbers(s: &str) -> HashSet<u32> {
        s.split(' ').filter_map(|s| s.trim().parse::<u32>().ok())
            .collect()
    }
    let winning_numbers = get_numbers(winning_numbers_and_my_numbers.next().unwrap());
    let my_numbers = get_numbers(winning_numbers_and_my_numbers.next().unwrap());
    let matches = my_numbers.intersection(&winning_numbers).count();
    if matches == 0 {
        0
    } else {
        2u32.pow((matches - 1) as u32)
    }
}

#[test]
fn test_check_scratchcard () {
    assert_eq!(check_scratchcard("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 8);
    assert_eq!(check_scratchcard("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
    assert_eq!(check_scratchcard("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
    assert_eq!(check_scratchcard("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
    assert_eq!(check_scratchcard("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
    assert_eq!(check_scratchcard("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
}

fn process_lines_in_file(filename: &str) -> u32 {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        sum += check_scratchcard(&line);
    }
    sum
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let sum = process_lines_in_file(filename);
    println!("{sum}");
}
