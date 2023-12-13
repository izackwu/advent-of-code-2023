use std::io::BufRead;

fn power_of_cubes(line: &str) -> u32 {
    let without_game_id = line[line.find(':').unwrap() + 1..].trim();
    let mut color_to_count = std::collections::HashMap::new();

    without_game_id.split(';').for_each(|colors_and_counts| {
        colors_and_counts.split(',').for_each(|color_and_count| {
            let count_and_color = color_and_count.trim().split(" ").collect::<Vec<&str>>();
            let count = count_and_color[0].parse::<u32>().unwrap();
            let color = count_and_color[1];
            color_to_count
                .entry(color)
                .and_modify(|e| *e = std::cmp::max(*e, count))
                .or_insert(count);
        });
    });
    if color_to_count.len() == 3 {
        color_to_count.values().fold(1, |acc, count| acc * count)
    } else {
        // if some color is missing, then by definition, we only need zero of that color and the product is zero
        0
    }
}

#[test]
fn test_power_of_cubes() {
    assert_eq!(
        power_of_cubes("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        4 * 2 * 6
    );
    assert_eq!(
        power_of_cubes("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
        1 * 3 * 4
    );
    assert_eq!(
        power_of_cubes("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
        20 * 13 * 6
    );
    assert_eq!(
        power_of_cubes("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
        14 * 3 * 15
    );
    assert_eq!(
        power_of_cubes("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        6 * 3 * 2
    );
}

fn process_lines_in_file(filename: &str) -> u32 {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        sum += power_of_cubes(&line);
    }
    sum
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let sum = process_lines_in_file(filename);
    println!("Sum of possible game IDs in {filename}: {}", sum);
}
