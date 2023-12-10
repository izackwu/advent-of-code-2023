use std::io::BufRead;

// This function is not very efficient, but the input data is simple enough that it works reasonably fast.
fn find_first_digit(line: &str, substrs_to_find: &[&str]) -> u32 {
    let mut digit  = 0;
    let mut min_pos = line.len();
    for (i, substr) in substrs_to_find.iter().enumerate() {
        match line.find(substr) {
            Some(pos) => {
                if pos < min_pos {
                    digit = i % 10;
                    min_pos = pos;
                }
            }
            None => (),
        }
    }
    digit as u32
}

// Given a line, find the first digit and the last digit (can be the same one), and combine them to form a two-digit number.
// Note: English words (e.g. one, two, ..., nine) are used for digits as well as 1, 2, .. 9.
fn calibration_value(line: &str) -> u32 {
    // substrs_to_find[i] = i % 10 (note that we don't expect zero or 0)
    let substrs_to_find = [
        "###", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "###", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
        "###", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"
    ];

    let first_digit = find_first_digit(&line, &substrs_to_find[0..20]);

    // reverse the line and every substring in substrs_to_find
    let reversed_line = line.chars().rev().collect::<String>();

    let last_digit = find_first_digit(&reversed_line, &substrs_to_find[10..30]);
    first_digit * 10 + last_digit
}

#[test]
fn test_calibration_value() {
    assert_eq!(calibration_value("two1nine"), 29);
    assert_eq!(calibration_value("eightwothree"), 83);
    assert_eq!(calibration_value("abcone2threexyz"), 13);
    assert_eq!(calibration_value("xtwone3four"), 24);
    assert_eq!(calibration_value("4nineeightseven2"), 42);
    assert_eq!(calibration_value("zoneight234"), 14);
    assert_eq!(calibration_value("7pqrstsixteen"), 76);
}

fn process_lines_in_file(filename: &str) -> u32 {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        sum += calibration_value(&line);
    }
    sum
}

#[test]
fn test_process_lines_in_file() {
    assert_eq!(process_lines_in_file("./src/input.txt"), 53894);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let sum = process_lines_in_file(filename);
    println!("Sum of calibration values in {filename}: {}", sum);
}
