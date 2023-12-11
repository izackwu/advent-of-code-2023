use std::collections::HashSet;

fn check_scratchcards(cards: &Vec<&str>) -> u32 {
    let check_one_card = |no: usize| {
        let line = cards[no];
        let without_card_id = line[line.find(':').unwrap() + 1..].trim();
        let mut winning_numbers_and_my_numbers = without_card_id.split('|');
        fn get_numbers(s: &str) -> HashSet<u32> {
            s.split(' ')
                .filter_map(|s| s.trim().parse::<u32>().ok())
                .collect()
        }
        let winning_numbers = get_numbers(winning_numbers_and_my_numbers.next().unwrap());
        let my_numbers = get_numbers(winning_numbers_and_my_numbers.next().unwrap());
        my_numbers.intersection(&winning_numbers).count()
    };
    let cards_cnt = cards.len();
    let mut copies = (0..cards_cnt).map(|_| 1).collect::<Vec<u32>>();
    for i in 0..cards_cnt {
        let matches = check_one_card(i);
        for j in i + 1..(i + matches + 1).min(cards_cnt) {
            copies[j] += copies[i];
        }
    }
    copies.iter().sum()
}

#[test]
fn test_check_scratchcards() {
    let cards = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];
    assert_eq!(check_scratchcards(&cards), 30);
}

fn process_lines_in_file(filename: &str) -> u32 {
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    check_scratchcards(&lines)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let sum = process_lines_in_file(filename);
    println!("{sum}");
}
