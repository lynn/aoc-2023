use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Xtreme,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    fn parse(letter: &str) -> Self {
        match letter {
            "x" => Self::Xtreme,
            "m" => Self::Musical,
            "a" => Self::Aerodynamic,
            "s" => Self::Shiny,
            _ => panic!("unknown category"),
        }
    }
}

#[derive(Debug)]
struct Condition {
    category: Category,
    ordering: Ordering,
    value: i64,
}

impl Condition {
    fn parse(string: &str) -> Self {
        let category = Category::parse(&string[0..1]);
        let ordering = match &string[1..2] {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            _ => panic!("unknown ordering"),
        };
        let value = string[2..].parse().unwrap();
        Self {
            category,
            ordering,
            value,
        }
    }

    fn matches(&self, part: &Part) -> bool {
        part.rating(self.category).cmp(&self.value) == self.ordering
    }

    fn threshold(&self) -> i64 {
        match self.ordering {
            Ordering::Less => self.value,
            _ => self.value + 1,
        }
    }
}

// a<2006:qkq
#[derive(Debug)]
enum Rule {
    If(Condition, String),
    Goto(String),
}

impl Rule {
    fn parse(string: &str) -> Self {
        match string.split_once(':') {
            Some((c, w)) => Rule::If(Condition::parse(c), w.to_owned()),
            None => Rule::Goto(string.to_owned()),
        }
    }

    fn matches(&self, part: &Part) -> bool {
        match self {
            Rule::If(c, _) => c.matches(part),
            Rule::Goto(_) => true,
        }
    }

    fn destination(&self) -> &str {
        match self {
            Rule::If(_, s) | Rule::Goto(s) => &s,
        }
    }
}

// a<2006:qkq,m>2090:A,rfg
#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Self {
        Self {
            rules: line.split(',').map(Rule::parse).collect(),
        }
    }

    fn destination(&self, part: &Part) -> &str {
        for rule in &self.rules {
            if rule.matches(part) {
                return rule.destination();
            }
        }
        panic!("no rule matched");
    }

    fn thresholds(&self, category: Category) -> HashSet<i64> {
        let mut set = HashSet::new();
        for rule in &self.rules {
            if let Rule::If(c, _) = rule {
                if c.category == category {
                    set.insert(c.threshold());
                }
            }
        }
        set
    }
}

#[derive(Debug)]
struct Program {
    workflows: HashMap<String, Workflow>,
}

impl Program {
    fn parse(input: &str) -> Self {
        Self {
            workflows: input
                .split('\n')
                .map(|line| {
                    let (name, rest) = line.split_once('{').unwrap();
                    let workflow = Workflow::parse(rest.trim_end_matches('}'));
                    (name.to_string(), workflow)
                })
                .collect(),
        }
    }

    fn accepts(&self, part: &Part) -> bool {
        let mut state = "in";
        while state.as_bytes()[0] >= b'a' {
            let workflow = self.workflows.get(state).expect("unknown workflow");
            state = workflow.destination(part);
        }
        state.as_bytes()[0] == b'A'
    }

    fn thresholds(&self, category: Category) -> HashSet<i64> {
        let mut set = HashSet::new();
        for workflow in self.workflows.values() {
            set.extend(&workflow.thresholds(category))
        }
        set
    }
}

#[derive(Debug)]
struct Part([i64; 4]);

impl Part {
    fn parse(line: &str) -> Self {
        Self(
            line[1..line.len() - 1]
                .split(',')
                .map(|p| p[2..].parse::<i64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    fn rating(&self, category: Category) -> i64 {
        self.0[category as usize]
    }

    fn total_rating(&self) -> i64 {
        self.0.iter().sum()
    }
}

pub fn main(input: &str) {
    let (program, parts) = input.split_once("\n\n").unwrap();
    let program = Program::parse(program);
    let parts: Vec<Part> = parts.split('\n').map(Part::parse).collect();

    let accepted_rating_sum: i64 = parts
        .iter()
        .filter(|p| program.accepts(p))
        .map(|p| p.total_rating())
        .sum();

    println!("*  {accepted_rating_sum}");

    let thresholds = |category: Category| -> Vec<(i64, i64)> {
        let mut t: Vec<i64> = program.thresholds(category).into_iter().collect();
        t.push(1);
        t.sort();
        t.iter()
            .zip(t[1..].iter().chain(&[4001]))
            .map(|(x, y)| (*x, *y))
            .collect()
    };

    // This little maneuver is gonna cost us 20 minutes. Merry Christmas!!!
    let mut total = 0;
    for (x0, x1) in thresholds(Category::Xtreme) {
        for (m0, m1) in thresholds(Category::Musical) {
            for (a0, a1) in thresholds(Category::Aerodynamic) {
                for (s0, s1) in thresholds(Category::Shiny) {
                    let count = (x1 - x0) * (m1 - m0) * (a1 - a0) * (s1 - s0);
                    if program.accepts(&Part([x0, m0, a0, s0])) {
                        total += count;
                    }
                }
            }
        }
    }

    println!("** {total}");
}
