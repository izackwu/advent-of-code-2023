fn total_winnings(mut hands_and_bids: Vec<(&str, u64)>) -> u64 {
    fn card_to_number(card: char) -> u8 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap() as u8,
        }
    }
    fn get_sort_key(hand: &str) -> (u8, [u8; 5]) {
        // convert hand to [char; 5]
        assert!(hand.len() == 5);
        let mut numbers = [0; 5];
        for (i, card) in hand.chars().enumerate() {
            numbers[i] = card_to_number(card);
        }
        let mut counts = [0; 15];
        for card in numbers.iter() {
            counts[*card as usize] += 1;
        }
        counts.sort();
        counts.reverse();
        // five of a kind, four of a kind, full house, three of a kind, two pair, one pair, high card
        (
            match true {
                _ if counts[0] == 5 => 6,
                _ if counts[0] == 4 => 5,
                _ if counts[0] == 3 && counts[1] == 2 => 4,
                _ if counts[0] == 3 => 3,
                _ if counts[0] == 2 && counts[1] == 2 => 2,
                _ if counts[0] == 2 => 1,
                _ => 0,
            },
            numbers,
        )
    }
    hands_and_bids.sort_by_cached_key(|(hand, _)| get_sort_key(hand));
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| ((i + 1) as u64) * (*bid))
        .sum()
}

#[test]
fn test_total_winnings() {
    let hands_and_bids = vec![
        ("32T3K", 765),
        ("T55J5", 684),
        ("KK677", 28),
        ("KTJJT", 220),
        ("QQQJA", 483),
    ];
    assert_eq!(total_winnings(hands_and_bids), 6440)
}

fn process_file(filename: &str) -> u64 {
    let content = std::fs::read_to_string(filename).unwrap();
    let hands_and_bids = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next()?;
            let bid = parts.next().and_then(|bid| bid.parse::<u64>().ok())?;
            Some((hand, bid))
        })
        .collect::<Vec<(&str, u64)>>();
    total_winnings(hands_and_bids)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let ans = process_file(filename);
    println!("{ans}");
}
