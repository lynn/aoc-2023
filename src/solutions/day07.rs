#[derive(Clone, Copy)]
enum Mode {
    Jacks,
    Jokers,
}

#[derive(Clone, Copy, Debug)]
struct Card {
    jack_value: usize,
    joker_value: usize,
    label: char,
}

const JACK_ALPHABET: &str = "23456789TJQKA";
const JOKER_ALPHABET: &str = "J23456789TQKA";
const CARD_TYPES: usize = JACK_ALPHABET.len();

impl From<char> for Card {
    fn from(label: char) -> Self {
        Self {
            label,
            jack_value: JACK_ALPHABET.find(label).expect("strange card"),
            joker_value: JOKER_ALPHABET.find(label).expect("strange card"),
        }
    }
}

impl Card {
    fn value(&self, mode: Mode) -> usize {
        match mode {
            Mode::Jacks => self.jack_value,
            Mode::Jokers => self.joker_value,
        }
    }

    fn is_joker(&self, mode: Mode) -> bool {
        matches!(mode, Mode::Jokers) && self.label == 'J'
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Ranking {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn parse(string: &str) -> Self {
        Self {
            cards: string.chars().map(Card::from).collect(),
        }
    }

    fn ranking(&self, mode: Mode) -> Ranking {
        let mut counts = [0; CARD_TYPES];
        let mut jokers = 0;
        for card in &self.cards {
            if card.is_joker(mode) {
                jokers += 1;
            } else {
                counts[card.value(mode)] += 1;
            }
        }
        counts.sort();
        // Count jokers as the most common non-joker card in the hand:
        counts[CARD_TYPES - 1] += jokers;

        let i = counts.partition_point(|&x| x == 0);
        match &counts[i..] {
            [5] => Ranking::FiveOfAKind,
            [1, 4] => Ranking::FourOfAKind,
            [2, 3] => Ranking::FullHouse,
            [1, 1, 3] => Ranking::ThreeOfAKind,
            [1, 2, 2] => Ranking::TwoPair,
            [1, 1, 1, 2] => Ranking::OnePair,
            [1, 1, 1, 1, 1] => Ranking::HighCard,
            _ => panic!("impossible counts: {:?}", &counts[i..]),
        }
    }

    /// Return a tie-breaker value to compare hands with, based on the cards in
    /// the hand. We turn a hand like `T55J5` into a number like `0x83393` (in
    /// jacks mode) or `0x94404` (in jokers mode).
    fn card_score(&self, mode: Mode) -> usize {
        self.cards.iter().fold(0, |a, c| 16 * a + c.value(mode))
    }

    fn strength(&self, mode: Mode) -> (Ranking, usize) {
        (self.ranking(mode), self.card_score(mode))
    }
}

struct Player {
    hand: Hand,
    bid: usize,
}

impl Player {
    fn parse(line: &str) -> Self {
        let (hand, bid) = line.split_once(' ').expect("bad line");
        Self {
            hand: Hand::parse(hand),
            bid: bid.parse().expect("bad bid"),
        }
    }
}

struct Session {
    players: Vec<Player>,
}

impl Session {
    fn parse(input: &str) -> Self {
        Self {
            players: input.split('\n').map(Player::parse).collect(),
        }
    }

    fn total_winnings(&mut self, mode: Mode) -> usize {
        self.players.sort_by_cached_key(|p| p.hand.strength(mode));
        self.players.iter().zip(1..).map(|(p, r)| p.bid * r).sum()
    }
}

pub fn main(input: &str) {
    let mut session = Session::parse(input);
    println!("*  {}", session.total_winnings(Mode::Jacks));
    println!("** {}", session.total_winnings(Mode::Jokers));
}
