use std::collections::HashMap;

pub struct Schematic<'a> {
    string: &'a [u8],
    width: usize,
    part_number_sum: u32,
    gears: HashMap<usize, Vec<u32>>,
}

impl<'a> Schematic<'a> {
    fn new(string: &'a str) -> Schematic<'a> {
        let mut schematic = Self {
            string: string.as_bytes(),
            width: string.find('\n').unwrap() + 1,
            part_number_sum: 0,
            gears: HashMap::new(),
        };

        for i in 0..string.len() {
            schematic.process_number_at(i);
        }

        schematic
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

    fn process_number_at(&mut self, index: usize) {
        if index > 0 && self.string[index - 1].is_ascii_digit() {
            // This is the *middle* of a part number, so don't process here.
            return;
        }
        let mut end = index;
        let mut value = 0;
        while end < self.string.len() && self.string[end].is_ascii_digit() {
            value = 10 * value + (self.string[end] - b'0') as u32;
            end += 1;
        }
        if index == end {
            // There's no part number here.
            return;
        }

        // Iterate over the cells around the number:
        //
        // .....
        // .123.
        // .....
        //
        let mut saw_symbol = false;
        let width = self.width as isize;
        for dy in [-width, width] {
            for x in index as isize - 1..=end as isize {
                if self.symbol_at(x + dy) {
                    saw_symbol = true;
                    self.count_gear((x + dy) as usize, value);
                }
            }
        }
        for j in [index as isize - 1, end as isize] {
            if self.symbol_at(j) {
                saw_symbol = true;
                self.count_gear(j as usize, value)
            }
        }

        if saw_symbol {
            self.part_number_sum += value;
        }
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
    let schematic = Schematic::new(input);

    println!("*  {}", schematic.part_number_sum);
    println!("** {}", schematic.gear_ratio_sum());
}
