use std::collections::HashMap;

pub struct Schematic<'a> {
    string: &'a [u8],
    width: usize,
    gears: HashMap<usize, Vec<u32>>,
}

impl<'a> Schematic<'a> {
    fn new(string: &'a str) -> Schematic<'a> {
        Self {
            string: string.as_bytes(),
            width: string.find('\n').unwrap() + 1,
            gears: HashMap::new(),
        }
    }

    fn symbol_at(&self, index: isize) -> bool {
        if index < 0 || index as usize >= self.string.len() {
            return false;
        }
        let c = self.string[index as usize];
        c != b'.' && c != b'\n'
    }

    fn count_gear(&mut self, index: usize, value: u32) {
        self.gears.entry(index).or_default().push(value);
    }

    fn part_number_at(&mut self, index: usize) -> Option<u32> {
        if index > 0 && self.string[index - 1].is_ascii_digit() {
            // This is the *middle* of a part number, so let's ignore it.
            return None;
        }
        let mut end = index;
        let mut value = 0;
        while end < self.string.len() && self.string[end].is_ascii_digit() {
            value = 10 * value + (self.string[end] - b'0') as u32;
            end += 1;
        }
        if index == end {
            return None;
        }

        // Iterate over the cells around the number:
        //
        // .....
        // .123.
        // .....
        //
        let mut result = None;
        let width = self.width as isize;
        for dy in [-width, width] {
            for x in index as isize - 1..=end as isize {
                if self.symbol_at(x + dy) {
                    result = Some(value);
                    self.count_gear((x + dy) as usize, value);
                }
            }
        }
        for j in [index as isize - 1, end as isize] {
            if self.symbol_at(j) {
                result = Some(value);
                self.count_gear(j as usize, value)
            }
        }

        result
    }

    fn part_number_sum(&mut self) -> u32 {
        (0..self.string.len())
            .filter_map(|i| self.part_number_at(i))
            .sum()
    }

    fn gear_ratio_sum(&self) -> u32 {
        self.gears
            .values()
            .filter(|x| x.len() == 2)
            .map(|x| x.iter().product::<u32>())
            .sum()
    }
}

pub fn main(input: &str) {
    let mut schematic = Schematic::new(input);

    println!("*  {}", schematic.part_number_sum());
    println!("** {}", schematic.gear_ratio_sum());
}
