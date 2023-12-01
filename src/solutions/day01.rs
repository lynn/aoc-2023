const SPELLED_DIGITS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_digit(line: &str) -> Option<u32> {
    SPELLED_DIGITS
        .iter()
        .find(|(word, _value)| line.starts_with(word))
        .map(|(_word, value)| *value)
        .or(line[..1].parse().ok())
}

fn simple_calibration(line: &str) -> Option<u32> {
    let first = line.bytes().find_map(|c| (c as char).to_digit(10));
    let last = line.bytes().rev().find_map(|c| (c as char).to_digit(10));
    Some(10 * first? + last?)
}

fn spelled_calibration(line: &str) -> Option<u32> {
    let first = (0..line.len()).find_map(|i| parse_digit(&line[i..]));
    let last = (0..line.len()).rev().find_map(|i| parse_digit(&line[i..]));
    Some(10 * first? + last?)
}

pub fn main(input: &str) {
    let mut total_simple = 0;
    let mut total_spelled = 0;

    for line in input.split('\n') {
        total_simple += simple_calibration(line).unwrap();
        total_spelled += spelled_calibration(line).unwrap();
    }

    println!("*  {total_simple}");
    println!("** {total_spelled}");
}
