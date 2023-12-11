fn process_content(lines: &Vec<&str>) -> u32 {
    let h: i32 = lines.len().try_into().unwrap();
    let w: i32 = lines[0].len().try_into().unwrap();
    let get = |i: i32, j: i32| {
        if i < 0 || i >= h || j < 0 || j >= w {
            return '.';
        }
        lines[i as usize].chars().nth(j as usize).unwrap()
    };
    let is_symbol = |i: i32, j: i32| -> bool {
        let c = get(i, j);
        !c.is_digit(10) && c != '.'
    };
    let is_digit = |i: i32, j: i32| get(i, j).is_digit(10);
    let is_adjacent_to_symbols = |row: i32, col_from: i32, col_to: i32| {
        let previous_line = (col_from - 1..col_to + 2).any(|j| is_symbol(row - 1, j));
        let current_line = is_symbol(row, col_from - 1) || is_symbol(row, col_to + 1);
        let next_line = (col_from - 1..col_to + 2).any(|j| is_symbol(row + 1, j));
        previous_line || current_line || next_line
    };
    let mut sum = 0;
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
            // check if number has any adjacent symbols
            if is_adjacent_to_symbols(i, j0, j - 1) {
                sum += number;
            }
        }
    }
    sum
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
    assert_eq!(process_content(&lines), 4361);
}

fn process_file(filename: &str) -> u32 {
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
