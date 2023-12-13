use std::io::BufRead;

// Given a line, find the first digit and the last digit (can be the same one), and combine them to form a two-digit number.
fn calibration_value(line: &str) -> u32 {
    let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
    let last_digit = line.chars().rfind(char::is_ascii_digit).unwrap();
    let first_digit = first_digit.to_digit(10).unwrap();
    let last_digit = last_digit.to_digit(10).unwrap();
    first_digit * 10 + last_digit
}

#[test]
fn test_calibration_value() {
    assert_eq!(calibration_value("1abc2"), 12);
    assert_eq!(calibration_value("pqr3stu8vwx"), 38);
    assert_eq!(calibration_value("a1b2c3d4e5f"), 15);
    assert_eq!(calibration_value("treb7uchet"), 77);
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let sum = process_lines_in_file(filename);
    println!("Sum of calibration values in {filename}: {}", sum);
}
