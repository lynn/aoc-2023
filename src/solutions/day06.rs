struct Race {
    time: f64,
    record: f64,
}

impl Race {
    fn ways_to_win(&self) -> i64 {
        // How many integers x are there in [0..t] such that x(t-x) > r?
        //
        // We can write that as: -x² + tx - r > 0. This is a quadratic with
        // discriminant d = b² - 4ac.
        let a = -1.0;
        let b = self.time;
        let c = -self.record;
        let d = b * b - 4.0 * a * c;

        // The roots are x = (-b ± sqrt(d)) / 2a:
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);

        // Because 2a is negative, we have x1 < x2. We do some tricky math to get
        // the first and last integers in the *open* interval (x1, x2).
        let x1 = x1.floor() as i64 + 1;
        let x2 = x2.ceil() as i64 - 1;

        // There are x2 - x1 + 1 integers in the range [x1..x2].
        x2 - x1 + 1
    }
}

struct RaceSheet {
    small: Vec<Race>,
    big: Race,
}

impl RaceSheet {
    fn parse(input: &str) -> Self {
        Self {
            small: Self::parse_small(input),
            big: Self::parse_big(input),
        }
    }

    fn parse_small(input: &str) -> Vec<Race> {
        let mut lines = input.split('\n').map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse::<f64>().unwrap())
        });

        let times = lines.next().unwrap();
        let records = lines.next().unwrap();
        times
            .zip(records)
            .map(|(time, record)| Race { time, record })
            .collect()
    }

    fn parse_big(input: &str) -> Race {
        let mut without_spaces = input.split('\n').map(|line| {
            line.chars()
                .filter(|x| x.is_ascii_digit())
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
        });

        let time = without_spaces.next().unwrap();
        let record = without_spaces.next().unwrap();
        Race { time, record }
    }

    fn small_answer(&self) -> i64 {
        self.small.iter().map(|race| race.ways_to_win()).product()
    }

    fn big_answer(&self) -> i64 {
        self.big.ways_to_win()
    }
}

pub fn main(input: &str) {
    let sheet = RaceSheet::parse(input);
    println!("*  {}", sheet.small_answer());
    println!("** {}", sheet.big_answer());
}
