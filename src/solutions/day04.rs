use std::collections::HashSet;

struct Card {
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (_, post) = line.split_once(": ").expect("no colon");
        let (winning, have) = post.split_once(" | ").expect("no line");
        Card {
            winning: winning
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
            have: have
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect(),
        }
    }

    fn matches(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }

    fn points(&self) -> u32 {
        (1 << self.matches()) >> 1
    }
}

struct Deck {
    cards: Vec<(usize, Card)>,
}

impl Deck {
    fn parse(input: &str) -> Self {
        Deck {
            cards: input
                .split('\n')
                .map(|line| (1, Card::parse(line)))
                .collect(),
        }
    }

    fn point_total(&self) -> u32 {
        self.cards.iter().map(|(_, card)| card.points()).sum()
    }

    fn copy_total(mut self) -> usize {
        let mut total = 0;
        for i in 0..self.cards.len() {
            let n = self.cards[i].0;
            let m = self.cards[i].1.matches();
            for j in i + 1..i + 1 + m {
                self.cards[j].0 += n;
            }
            total += n;
        }
        total
    }
}

pub fn main(input: &str) {
    let deck = Deck::parse(input);
    println!("*  {}", deck.point_total());
    println!("*  {}", deck.copy_total());
}
