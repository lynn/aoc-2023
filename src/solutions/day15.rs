fn hash(text: &str) -> usize {
    text.bytes().fold(0, |acc, b| (acc + b as usize) * 17 % 256)
}

enum Step<'a> {
    Set(&'a str, u8),
    Remove(&'a str),
}

impl<'a> Step<'a> {
    fn parse(string: &'a str) -> Self {
        if let Some((pre, post)) = string.split_once('=') {
            Step::Set(pre, post.parse().unwrap())
        } else if let Some(label) = string.strip_suffix('-') {
            Step::Remove(label)
        } else {
            panic!("can't parse step {string}")
        }
    }

    fn label(&'a self) -> &'a str {
        match self {
            Step::Set(l, _) | Step::Remove(l) => l,
        }
    }
}

#[derive(Debug, Default)]
struct LensBox {
    slots: Vec<(String, u8)>,
}

impl LensBox {
    fn execute<'a>(&mut self, step: Step<'a>) {
        match step {
            Step::Set(label, value) => self.set(label, value),
            Step::Remove(label) => self.remove(label),
        }
    }

    fn set(&mut self, label: &str, value: u8) {
        match self.slots.iter_mut().find(|s| s.0 == label) {
            Some(slot) => slot.1 = value,
            None => self.slots.push((label.to_owned(), value)),
        }
    }

    fn remove(&mut self, label: &str) {
        self.slots.retain(|s| s.0 != label);
    }

    fn focusing_power(&self, box_number: usize) -> usize {
        self.slots
            .iter()
            .zip(1..)
            .map(|(slot, slot_number)| box_number * slot_number * (slot.1 as usize))
            .sum()
    }
}

#[derive(Debug)]
struct LensMap {
    boxes: [LensBox; 256],
}

impl LensMap {
    fn new() -> Self {
        Self {
            boxes: std::array::from_fn(|_| Default::default()),
        }
    }

    fn execute<'a>(&mut self, step: Step<'a>) {
        self.boxes[hash(step.label())].execute(step);
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .zip(1..)
            .map(|(b, i)| b.focusing_power(i))
            .sum()
    }
}

pub fn main(input: &str) {
    let mut total = 0;
    let mut lens_map = LensMap::new();

    for step in input.split(',') {
        total += hash(step);
        lens_map.execute(Step::parse(step));
    }
    println!("*  {total}");
    println!("** {}", lens_map.focusing_power());
}
