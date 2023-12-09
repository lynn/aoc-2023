#[derive(Default)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn empty() -> Self {
        Self::default()
    }

    /// Parse a string like `6 green, 15 red` into a bag.
    fn parse(string: &str) -> Self {
        let mut bag = Self::empty();
        for entry in string.split(", ") {
            let (count, color) = entry.split_once(' ').expect("bad entry");
            let count = count.parse::<u32>().expect("bad count");
            match color {
                "red" => bag.red = count,
                "green" => bag.green = count,
                "blue" => bag.blue = count,
                _ => panic!("unknown color {color}"),
            }
        }
        bag
    }

    fn is_subset(&self, other: &Bag) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn max(self, other: &Bag) -> Bag {
        Bag {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    turns: Vec<Bag>,
}

impl Game {
    /// Parse a string like `Game 3: 6 green, 15 red; 1 green, 4 red` into a game.
    fn parse(line: &str) -> Self {
        let (head, body) = line.split_once(": ").expect("no colon");
        let (_game, id) = head.split_once(' ').expect("bad header");
        let id = id.parse::<u32>().expect("bad ID");
        let turns = body.split("; ").map(Bag::parse).collect();
        Game { id, turns }
    }

    fn is_possible(&self, full: &Bag) -> bool {
        self.turns.iter().all(|b| b.is_subset(full))
    }

    fn minimum_bag(&self) -> Bag {
        self.turns.iter().fold(Bag::empty(), Bag::max)
    }
}

pub fn main(input: &str) {
    let mut id_sum = 0;
    let mut power_sum = 0;
    let elf_bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    for line in input.split('\n') {
        let game = Game::parse(line);

        if game.is_possible(&elf_bag) {
            id_sum += game.id;
        }

        power_sum += game.minimum_bag().power()
    }

    println!("*  {id_sum}");
    println!("** {power_sum}");
}
