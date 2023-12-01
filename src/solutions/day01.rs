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

fn parse_spelled_digit(line: &str) -> Option<u32> {
    SPELLED_DIGITS
        .iter()
        .find(|(word, _value)| line.starts_with(word))
        .map(|(_word, value)| *value)
}

struct CalibrationValues {
    simple: u32,
    spelled: u32,
}

fn calibration_values(line: &str) -> Option<CalibrationValues> {
    let mut first_simple: Option<u32> = None;
    let mut last_simple: Option<u32> = None;
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for (i, c) in line.chars().enumerate() {
        if let Some(value) = parse_spelled_digit(&line[i..]) {
            first = first.or(Some(value));
            last = Some(value);
        } else if let Some(value) = c.to_digit(10) {
            first_simple = first_simple.or(Some(value));
            last_simple = Some(value);
            first = first.or(Some(value));
            last = Some(value);
        }
    }

    Some(CalibrationValues {
        simple: 10 * first_simple? + last_simple?,
        spelled: 10 * first? + last?,
    })
}

pub fn main(input: &str) {
    let mut total_simple = 0;
    let mut total_spelled = 0;

    for line in input.split('\n') {
        let values = calibration_values(line).unwrap();
        total_simple += values.simple;
        total_spelled += values.spelled;
    }

    println!("*  {total_simple}");
    println!("** {total_spelled}");
}
