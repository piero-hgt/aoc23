use std::{fs, collections::HashMap, cmp::Ordering};

const CARDS_STRENGTH: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandStrength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn new(input: &str) -> Self {
        let hand = input.split_whitespace().nth(0).unwrap();
        let bid = input.split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();

        let mut cards = Vec::new();
        for c in hand.chars() {
            cards.push(Card::new(c));
        }
        Hand { cards, bid }
    }

    fn strength(&self) -> HandStrength {
        let mut cards_map: HashMap<char, u32> = HashMap::new();

        for card in &self.cards {
            let count = cards_map.entry(card.value).or_insert(0);
            *count += 1;
        }

        // order hashmap by value desc
        let mut cards_map: Vec<(&char, &u32)> = cards_map.iter().collect();
        cards_map.sort_by(|a, b| b.1.cmp(a.1));

        for (&c, &count) in cards_map.iter() {
            match count {
                5 => return HandStrength::FiveOfAKind,
                4 => return HandStrength::FourOfAKind,
                3 => {
                    // Full house or threeofakind
                    if cards_map.len() == 2 {
                        return HandStrength::FullHouse;
                    }
                    return HandStrength::ThreeOfAKind;
                },
                2 => {
                    // Two pairs or one pair
                    if cards_map.len() == 3 {
                        return HandStrength::TwoPairs;
                    }
                    return HandStrength::OnePair;
                },
                _ => return HandStrength::HighCard,
            }
        }

        HandStrength::HighCard
    }

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.strength().cmp(&other.strength()))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength() == other.strength() {
            // println!("hand strength equality between {:?} and {:?}", self, other);
            for (i, card) in self.cards.iter().enumerate() {
                if card.get_strength() == other.cards[i].get_strength() {
                    continue;
                }

                if other.cards[i].get_strength() > card.get_strength() {
                    return Ordering::Greater;
                }
                return Ordering::Less;
            }
            return Ordering::Equal;
        }
        self.strength().cmp(&other.strength())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: char,
}

impl Card {
    fn new(value: char) -> Self {
        Card { value }
    }

    fn get_strength(&self) -> usize {
        CARDS_STRENGTH.iter().position(|&x| x == self.value).unwrap()
    }
}

fn sum_total_winnings(input: &str) -> u32 {
    let mut hands = input.lines().map(Hand::new).collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.cmp(&b));
    hands.reverse();
    // dbg!(&hands);

    let mut total_winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += (hand.bid * (i+1) as u32);
    }
    total_winnings
}


pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day07.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_total_winnings(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_total_winnings() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        assert_eq!(sum_total_winnings(input), 6440);
    }
}
