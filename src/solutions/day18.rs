#[derive(Clone, Copy)]
enum Mode {
    Regular,
    FromColor,
}

#[derive(Debug)]
struct Edge {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

#[derive(Debug)]
struct Lagoon {
    edges: Vec<Edge>,
}

impl Lagoon {
    fn inner_area(&self) -> i64 {
        let mut verticals: Vec<(i64, (i64, i64))> = self
            .edges
            .iter()
            .filter(|e| e.x1 == e.x2)
            .map(|e| (e.x1, (e.y1.min(e.y2), e.y1.max(e.y2))))
            .collect();
        verticals.sort();
        let mut area = 0;
        let mut sign = -1;

        let mut ys: Vec<i64> = self.edges.iter().map(|e| e.y1).collect();
        ys.sort();
        ys.dedup();

        for w in ys.windows(2) {
            let height = w[1] - w[0];
            for (x, (y1, y2)) in &verticals {
                if *y1 <= w[0] && w[0] < *y2 {
                    area += x * sign * height;
                    sign = -sign;
                }
            }
        }

        area
    }

    fn circumference(&self) -> i64 {
        self.edges
            .iter()
            .map(|e| e.x1.abs_diff(e.x2) + e.y1.abs_diff(e.y2))
            .sum::<u64>() as i64
    }

    fn area(&self) -> i64 {
        self.inner_area() + self.circumference() / 2 + 1
    }
}

struct Step {
    vector: (i64, i64),
    color: u32,
}

impl Step {
    fn parse(line: &str) -> Self {
        let (pre, post) = line.split_once(" (#").unwrap();
        let color = u32::from_str_radix(&post[..6], 16).unwrap();
        let length: i64 = pre[2..].parse().unwrap();
        let vector = match &pre[..1] {
            "R" => (length, 0),
            "D" => (0, length),
            "L" => (-length, 0),
            "U" => (0, -length),
            _ => panic!("direction"),
        };
        Self { vector, color }
    }

    fn vector(&self, mode: Mode) -> (i64, i64) {
        match mode {
            Mode::Regular => self.vector,
            Mode::FromColor => {
                let length = (self.color >> 4) as i64;
                match self.color & 0x3 {
                    0 => (length, 0),
                    1 => (0, length),
                    2 => (-length, 0),
                    _ => (0, -length),
                }
            }
        }
    }
}

struct Plan(Vec<Step>);

impl Plan {
    fn parse(input: &str) -> Self {
        Self(input.split('\n').map(Step::parse).collect())
    }

    fn execute(&self, mode: Mode) -> Lagoon {
        let mut edges = vec![];
        let mut x = 0;
        let mut y = 0;
        for step in &self.0 {
            let (dx, dy) = step.vector(mode);
            edges.push(Edge {
                x1: x,
                y1: y,
                x2: x + dx,
                y2: y + dy,
            });
            x += dx;
            y += dy;
        }
        Lagoon { edges }
    }
}

pub fn main(input: &str) {
    let plan = Plan::parse(input);
    println!("*  {:?}", plan.execute(Mode::Regular).area());
    println!("** {:?}", plan.execute(Mode::FromColor).area());
}
