use std::{fs, collections::BTreeMap};
use text_io::scan;

struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug, Clone)]
struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Card {
        let card: String;
        let winning: String;
        let mine: String;
        scan!(input.trim().bytes() => "{}: {}|{}\n", card, winning, mine);

        let number: u32 = card.trim().replace(' ', "").replace("Card", "").parse::<u32>().unwrap();
        let winning_numbers = winning.trim().split_whitespace().into_iter().map(|n| n.trim().parse::<u32>().unwrap()).collect();
        let my_numbers = mine.trim().split_whitespace().into_iter().map(|n| n.trim().parse::<u32>().unwrap()).collect();

        Card { number, winning_numbers, my_numbers }
    }

    fn get_my_winning_numbers(&self) -> Vec<u32> {
        self.my_numbers.iter().filter(|n| self.winning_numbers.contains(n)).map(|n| *n).collect()
    }

    fn get_score(&self) -> u32 {
        let mut score = 0;

        for n in &self.get_my_winning_numbers() {
            score = if score == 0 { 1 } else { score * 2 };
        }

        score
    }

    fn get_my_winning_numbers_count(&self) -> u32 {
        self.get_my_winning_numbers().len() as u32
    }
}


fn sum_winning_cards_points(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let card = Card::new(line);
        sum += card.get_score();
    }
    sum
}

fn count_final_cards(input: &str) -> u32 {
    let mut initial_cards: BTreeMap<u32, Card> = BTreeMap::new();
    let mut final_cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let card = Card::new(line);
        initial_cards.insert(card.number, card);
    }

    for (number, card) in initial_cards.iter() {
        let current_cards: Vec<Card> = generate_new_cards(&initial_cards, &card);
        for c in current_cards {
            final_cards.push(c.clone());
        }
    }
    final_cards.len() as u32
}

fn generate_new_cards(initial_cards: &BTreeMap<u32, Card>, card: &Card) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![card.clone()];

    let count_winning = card.get_my_winning_numbers_count();

    // println!("Card {} has {} winning numbers", card.number, count_winning);

    let range = (card.number + 1)..=(card.number + count_winning);

    for n in range {
        let new_card = initial_cards[&n].clone();
        // dbg!(&new_card);
        for c in generate_new_cards(initial_cards, &new_card) {
            cards.push(c);
        }
    }
    cards
}

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day04.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_winning_cards_points(&contents));
}

pub fn solve_task2() {
    let contents = match fs::read_to_string("input/day04.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", count_final_cards(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_winning_cards_points() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(sum_winning_cards_points(input), 13);
    }

    #[test]
    fn test_count_final_cards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(count_final_cards(input), 30);
    }
}
