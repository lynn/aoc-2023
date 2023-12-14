struct Configurations<'a> {
    template: &'a str,
    buffer: Vec<bool>,
    yielded_first: bool,
}

impl<'a> Configurations<'a> {
    fn new(template: &'a str) -> Self {
        Self {
            template,
            buffer: template.bytes().map(|b| b == b'#').collect(),
            yielded_first: false,
        }
    }
}

impl<'a> Iterator for Configurations<'a> {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.yielded_first {
            self.yielded_first = true;
            return Some(self.buffer.clone());
        }
        for (i, c) in self.template.bytes().enumerate() {
            if c == b'?' {
                if self.buffer[i] {
                    self.buffer[i] = false;
                } else {
                    self.buffer[i] = true;
                    return Some(self.buffer.clone());
                }
            }
        }
        return None;
    }
}

fn matches(config: &[bool], nums: &[usize]) -> bool {
    config
        .split(|b| !b)
        .map(|x| x.len())
        .filter(|x| *x > 0)
        .eq(nums.iter().map(|x| *x))
}

fn count_configurations(pattern: &str, nums: &[usize]) -> usize {
    let mut count = 0;
    for config in Configurations::new(pattern) {
        if matches(&config, nums) {
            count += 1;
        }
    }
    count
}

pub fn main(input: &str) {
    let mut sum = 0;
    let mut sum2 = 0;
    for line in input.split('\n') {
        let (pattern, nums) = line.split_once(' ').unwrap();
        let nums: Vec<usize> = nums.split(',').map(|x| x.parse().unwrap()).collect();
        let n = count_configurations(pattern, &nums);
        sum += n;
        // let unfolded = format!("{pattern}?{pattern}?{pattern}?{pattern}?{pattern}");
        // let n2 = count_configurations(&unfolded, &nums.repeat(5));
        // sum2 += n2;
        // println!("{n2}");
    }
    println!("*  {sum}");
    // println!("** {sum2}");
}
