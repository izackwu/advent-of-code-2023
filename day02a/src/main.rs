use std::io::BufRead;

fn color_to_count(color: &str) -> u32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0,
    }
}

// Format of a line: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn check_game(line: &str) -> u32 {
    let mut parts = line.split([';', ':', ',']);
    let game_number = parts.next().unwrap(); // something like "Game 1"
    let game_number: u32 = game_number.split(" ").last().unwrap().parse().unwrap(); // something like 1
    fn enough_cubes(cubes: &str) -> bool {
        let count_and_color = cubes.trim().split(" ").collect::<Vec<&str>>();
        let count = count_and_color[0].parse::<u32>().unwrap();
        count <= color_to_count(count_and_color[1])
    }
    if parts.all(enough_cubes) {
        game_number
    } else {
        0
    }
}

#[test]
fn test_check_game() {
    assert_eq!(check_game("Game 1: 1 green, 1 blue, 1 red; 3 green, 1 blue, 1 red; 4 green, 3 blue, 1 red; 4 green, 2 blue, 1 red; 3 blue, 3 green"), 1);
    assert_eq!(check_game("Game 2: 9 blue, 7 red; 5 blue, 6 green, 1 red; 2 blue, 10 red, 9 green; 3 green, 14 red, 5 blue; 8 red, 3 blue, 4 green; 8 green, 14 blue, 10 red"), 0);
    assert_eq!(check_game("Game 3: 11 green, 8 blue, 7 red; 3 green, 4 red, 9 blue; 3 red, 4 green, 8 blue; 11 green, 1 red, 16 blue"), 0);
    assert_eq!(check_game("Game 4: 3 blue, 20 green, 2 red; 1 green, 3 red, 3 blue; 1 blue, 9 green; 4 red, 17 green; 12 green, 3 red"), 0);
    assert_eq!(
        check_game("Game 5: 2 red, 1 blue, 4 green; 6 blue, 2 green; 2 red, 5 green"),
        5
    );
}

fn process_lines_in_file(filename: &str) -> u32 {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        sum += check_game(&line);
    }
    sum
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let sum = process_lines_in_file(filename);
    println!("Sum of possible game IDs in {filename}: {}", sum);
}
