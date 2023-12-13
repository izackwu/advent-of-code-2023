fn count_solutions(races: Vec<(u64, u64)>) -> u64 {
    races
        .into_iter()
        .map(|(time_int, distance_int)| {
            let time = time_int as f64;
            let distance = distance_int as f64;
            let lower_bound = (time - (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
            let upper_bound = (time + (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
            // find the number of integers strictly between lower_bound and upper_bound
            let lower_bound = lower_bound.ceil() as u64;
            let upper_bound = upper_bound.floor() as u64;
            if lower_bound * (time_int - lower_bound) == distance_int {
                upper_bound - lower_bound - 1
            } else {
                upper_bound - lower_bound + 1
            }
        })
        .into_iter()
        .product()
}

#[test]
fn test_count_solutions() {
    assert_eq!(
        count_solutions(vec![(7, 9), (15, 40), (30, 200)]),
        288
    );
    assert_eq!(
        count_solutions(vec![(71530, 940200)]),
        71503
    );

}

fn process_file(filename: &str) -> u64 {
    let content = std::fs::read_to_string(filename).unwrap();
    let lines = content.lines().collect::<Vec<&str>>();
    assert!(lines.len() == 2);
    let times = lines[0]
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let distances = lines[1]
        .split(' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    assert!(times.len() == distances.len());
    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .collect::<Vec<(u64, u64)>>();
    count_solutions(races)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let ans = process_file(filename);
    println!("{ans}");
}
