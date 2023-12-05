use gcollections::ops::*;
use interval::interval_set::*;

struct Conversion {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl Conversion {
    fn parse(line: &str) -> Self {
        let mut nums = line.split(' ').map(|x| x.parse::<i64>().unwrap());
        Self {
            destination_range_start: nums.next().unwrap(),
            source_range_start: nums.next().unwrap(),
            range_length: nums.next().unwrap(),
        }
    }

    fn offset(&self) -> i64 {
        self.destination_range_start - self.source_range_start
    }

    fn convert(&self, input: i64) -> Option<i64> {
        if self.source_range_start <= input && input < self.source_range_start + self.range_length {
            Some(input + self.offset())
        } else {
            None
        }
    }

    // **
    fn source_set(&self) -> IntervalSet<i64> {
        vec![(
            self.source_range_start,
            self.source_range_start + self.range_length - 1,
        )]
        .to_interval_set()
    }
}

struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn parse(paragraph: &str) -> Self {
        let lines = paragraph.split('\n').skip(1);
        Self {
            conversions: lines.map(Conversion::parse).collect(),
        }
    }

    fn convert(&self, input: i64) -> i64 {
        self.conversions
            .iter()
            .find_map(|c| c.convert(input))
            .unwrap_or(input)
    }

    // **
    fn convert_set(&self, mut set: IntervalSet<i64>) -> IntervalSet<i64> {
        let mut mapped: IntervalSet<i64> = IntervalSet::empty();
        for c in &self.conversions {
            let src = set.intersection(&c.source_set());
            set = set.difference(&src);
            let dst = src + c.offset();
            mapped = mapped.union(&dst);
        }
        set.union(&mapped)
    }
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let mut paragraphs = input.split("\n\n");
        let seed_paragraph = paragraphs.next().unwrap();
        let seeds = seed_paragraph
            .split(' ')
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let maps = paragraphs.map(Map::parse).collect();
        Self { seeds, maps }
    }

    fn convert(&self, seed: i64) -> i64 {
        self.maps.iter().fold(seed, |s, m| m.convert(s))
    }

    fn convert_set(&self, set: IntervalSet<i64>) -> IntervalSet<i64> {
        self.maps.iter().fold(set, |s, m| m.convert_set(s))
    }

    fn lowest_location(&self) -> i64 {
        self.seeds.iter().map(|s| self.convert(*s)).min().unwrap()
    }

    // **
    fn seed_set(&self) -> IntervalSet<i64> {
        self.seeds
            .chunks(2)
            .map(|x| vec![(x[0], x[0] + x[1] - 1)].to_interval_set())
            .fold(IntervalSet::empty(), |a, b| a.union(&b))
    }

    fn range_lowest_location(&self) -> i64 {
        self.convert_set(self.seed_set()).lower()
    }
}

pub fn main(input: &str) {
    let almanac = Almanac::parse(input);
    println!("*  {:?}", almanac.lowest_location());
    println!("** {:?}", almanac.range_lowest_location());
}
