use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug)]
enum Behavior {
    FlipFlop { on: bool },
    Conjunction { inputs: HashMap<String, Pulse> },
    Broadcast,
}

impl Behavior {
    fn process_pulse(&mut self, input: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Behavior::FlipFlop { on } if pulse == Pulse::Low => {
                if *on {
                    *on = false;
                    Some(Pulse::Low)
                } else {
                    *on = true;
                    Some(Pulse::High)
                }
            }
            Behavior::Conjunction { inputs } => {
                *inputs.get_mut(input).unwrap() = pulse;
                if inputs.values().all(|p| *p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Behavior::Broadcast => Some(pulse),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Module {
    behavior: Behavior,
    destinations: Vec<String>,
}

impl Module {
    fn process_pulse(&mut self, input: &str, pulse: Pulse) -> Option<Pulse> {
        self.behavior.process_pulse(input, pulse)
    }
}

#[derive(Clone, Debug)]
struct Circuit {
    modules: HashMap<String, Module>,
}

fn parse_name(name: &str) -> (Behavior, String) {
    match &name[..1] {
        "%" => (Behavior::FlipFlop { on: false }, name[1..].to_owned()),
        "&" => (
            Behavior::Conjunction {
                inputs: HashMap::new(),
            },
            name[1..].to_owned(),
        ),
        _ => (Behavior::Broadcast, name.to_owned()),
    }
}

struct Report {
    low_pulses: usize,
    low_pulses_to_rx: usize,
    high_pulses: usize,
}

impl Circuit {
    fn parse(input: &str) -> Self {
        let mut modules = HashMap::new();
        for line in input.split('\n') {
            let (before, after) = line.split_once(" -> ").unwrap();
            let (behavior, name) = parse_name(before);
            let destinations = after.split(", ").map(|d| d.to_owned()).collect();
            let module = Module {
                behavior,
                destinations,
            };
            modules.insert(name, module);
        }

        let names = modules.keys().cloned().collect::<Vec<_>>();
        for src_name in &names {
            for dst_name in modules.get(src_name).unwrap().destinations.clone() {
                if let Some(dst) = modules.get_mut(&dst_name) {
                    if let Behavior::Conjunction { inputs } = &mut dst.behavior {
                        inputs.insert(src_name.clone(), Pulse::Low);
                    }
                }
            }
        }
        Self { modules }
    }

    fn push_button(&mut self) -> Report {
        let mut pulse_queue =
            VecDeque::from([("button".to_owned(), Pulse::Low, "broadcaster".to_owned())]);
        let mut low_pulses = 0;
        let mut low_pulses_to_rx = 0;
        let mut high_pulses = 0;
        while let Some((s0, pulse, s1)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::Low => {
                    low_pulses += 1;
                    if s1 == "rx" {
                        low_pulses_to_rx += 1;
                    }
                }
                Pulse::High => high_pulses += 1,
            }
            // println!("{s0} {pulse:?} {s1}");
            if let Some(m1) = self.modules.get_mut(&s1) {
                let response = m1.process_pulse(&s0, pulse);
                if let Some(pulse) = response {
                    for s2 in &m1.destinations {
                        pulse_queue.push_back((s1.clone(), pulse, s2.clone()));
                    }
                }
            }
        }
        Report {
            low_pulses,
            low_pulses_to_rx,
            high_pulses,
        }
    }

    fn simulate(&mut self, steps: usize) -> usize {
        let mut low_pulses = 0;
        let mut high_pulses = 0;
        for _ in 0..steps {
            let report = self.push_button();
            low_pulses += report.low_pulses;
            high_pulses += report.high_pulses;
        }
        low_pulses * high_pulses
    }

    fn presses_until_rx(&mut self) -> usize {
        for n in 1.. {
            let report = self.push_button();
            if report.low_pulses_to_rx > 0 {
                return n;
            }
            for (k, v) in &self.modules {
                if let Behavior::FlipFlop { on } = v.behavior {
                    print!("{k}{} ", on as usize);
                }
            }
            println!()
        }
        unreachable!()
    }
}

pub fn main(input: &str) {
    let mut circuit = Circuit::parse(input);
    println!("*  {}", circuit.simulate(1000));

    // Ah, yes, and this takes forever. Sadly I have a job, ERIC
    println!("** {}", circuit.presses_until_rx());
}
