struct Extrapolation {
    next: i32,
    previous: i32,
}

fn extrapolate(mut nums: Vec<i32>) -> Extrapolation {
    let mut next = 0;
    let mut previous = 0;
    let mut sign = 1;
    while nums.iter().any(|x| *x != 0) {
        next += nums.last().unwrap();
        previous += nums.first().unwrap() * sign;
        sign = -sign;
        nums = nums.windows(2).map(|x| x[1] - x[0]).collect();
    }
    Extrapolation { next, previous }
}

pub fn main(input: &str) {
    let mut total_next = 0;
    let mut total_previous = 0;
    for line in input.split('\n') {
        let x = extrapolate(
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
        total_next += x.next;
        total_previous += x.previous;
    }
    println!("*  {}", total_next);
    println!("** {}", total_previous);
}
