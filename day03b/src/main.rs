use std::collections::HashMap;

fn process_content(lines: &Vec<&str>) -> u64 {
    let h: i32 = lines.len().try_into().unwrap();
    let w: i32 = lines[0].len().try_into().unwrap();
    let get = |i: i32, j: i32| {
        if i < 0 || i >= h || j < 0 || j >= w {
            return '.';
        }
        lines[i as usize].chars().nth(j as usize).unwrap()
    };
    let is_gear_symbol = |i: i32, j: i32| -> bool { get(i, j) == '*' };
    let is_digit = |i: i32, j: i32| get(i, j).is_digit(10);
    // find the coordinates of all adjacent gear symbols
    let adjacent_gear_symbols = |row: i32, col_from: i32, col_to: i32| {
        let f = |i: i32, j: i32| is_gear_symbol(i, j).then_some((i, j));
        let previous_line = (col_from - 1..col_to + 2).filter_map(|j| f(row - 1, j));
        let cols = [col_from - 1, col_to + 1];
        let current_line = cols.iter().filter_map(|j| f(row, *j));
        let next_line = (col_from - 1..col_to + 2).filter_map(|j| f(row + 1, j));
        previous_line
            .chain(current_line)
            .chain(next_line)
            .collect::<Vec<_>>()
    };
    // gear coordinates to the list of numbers adjacent to it
    let mut gears_to_numbers: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    for i in 0..h {
        let mut j = 0;
        while j < w {
            while j < w && !(is_digit(i, j)) {
                j += 1;
            }
            if j >= w {
                break;
            }
            // now lines[i][j] is a digit
            let j0 = j;
            let mut number = 0;
            while j < w && is_digit(i, j) {
                number = number * 10 + get(i, j).to_digit(10).unwrap();
                j += 1;
            }
            let adjacent_gears = adjacent_gear_symbols(i, j0, j - 1);
            adjacent_gears.iter().for_each(|(i, j)| {
                gears_to_numbers
                    .entry((*i, *j))
                    .and_modify(|v| v.push(number))
                    .or_insert(vec![number]);
            });
        }
    }
    gears_to_numbers.values().fold(0, |acc, v| {
        acc + if v.len() == 2 {
            (v[0] as u64) * (v[1] as u64)
        } else {
            0
        }
    })
}

#[test]
fn test_process_content() {
    let lines = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];
    assert_eq!(process_content(&lines), 467835);
}

fn process_file(filename: &str) -> u64 {
    // read all contents of file into a list of strings line by line
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    process_content(&lines)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let sum = process_file(filename);
    println!("{sum}")
}
