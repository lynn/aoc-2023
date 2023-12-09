use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node(u32);

impl Node {
    fn parse(node: &str) -> Self {
        let b = node.as_bytes();
        Self(u32::from_be_bytes([0, b[0], b[1], b[2]]))
    }

    fn is_ghost_entrance(&self) -> bool {
        self.0 % 256 == b'A' as u32
    }

    fn is_ghost_exit(&self) -> bool {
        self.0 % 256 == b'Z' as u32
    }
}

struct Desert<'a> {
    instructions: &'a str,
    network: HashMap<Node, (Node, Node)>,
}

#[derive(Debug)]
struct Loop {
    loop_length: usize,
    z_offsets: Vec<usize>,
}

impl<'a> Desert<'a> {
    fn parse(input: &'a str) -> Self {
        let (instructions, network) = input.split_once("\n\n").unwrap();
        let network = network
            .split('\n')
            .map(|line| {
                let src = Node::parse(&line[0..3]);
                let l = Node::parse(&line[7..10]);
                let r = Node::parse(&line[12..15]);
                (src, (l, r))
            })
            .collect();
        Self {
            instructions,
            network,
        }
    }

    fn step(&self, node: Node, direction: char) -> Node {
        let (left, right) = self.network.get(&node).unwrap();
        match direction {
            'L' => *left,
            _ => *right,
        }
    }

    fn find_loop(&self, start_node: Node) -> Loop {
        let mut node = start_node;
        let mut seen: HashMap<(Node, usize), usize> = HashMap::new();
        let mut z_offsets: Vec<usize> = vec![];
        for (i, c) in self.instructions.chars().cycle().enumerate() {
            if let Some(j) = seen.insert((node, i % self.instructions.len()), i) {
                return Loop {
                    loop_length: i - j,
                    z_offsets,
                };
            }
            if node.is_ghost_exit() {
                z_offsets.push(i)
            }
            node = self.step(node, c);
        }
        unreachable!()
    }

    fn people_steps(&self) -> usize {
        let mut node = Node::parse("AAA");
        let end = Node::parse("ZZZ");
        for (i, c) in self.instructions.chars().cycle().enumerate() {
            if node == end {
                return i;
            }
            node = self.step(node, c);
        }
        unreachable!()
    }

    fn ghost_steps(&self) -> usize {
        let loops = self
            .network
            .keys()
            .filter(|x| x.is_ghost_entrance())
            .map(|x| self.find_loop(*x));

        loops.fold(1, |acc, l| {
            // This happens to be true... and means we can just lcm the numbers
            // together intead of doing, idk, some chinese remainder theorem
            // thing?
            assert_eq!(l.z_offsets, vec![l.loop_length]);
            num::integer::lcm(acc, l.loop_length)
        })
    }
}

pub fn main(input: &str) {
    let desert = Desert::parse(input);
    println!("*  {}", desert.people_steps());
    println!("** {}", desert.ghost_steps());
}
