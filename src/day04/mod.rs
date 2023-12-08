use std::fs;
use text_io::scan;

#[derive(Debug)]
struct Card {
    // number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Card {
        let card: String;
        let winning: String;
        let mine: String;
        scan!(input.trim().bytes() => "{}: {}|{}\n", card, winning, mine);

        // let number: u32 = card.trim().replace(' ', "").replace("Game", "").parse::<u32>().unwrap();
        let winning_numbers = winning.trim().split_whitespace().into_iter().map(|n| n.trim().parse::<u32>().unwrap()).collect();
        let my_numbers = mine.trim().split_whitespace().into_iter().map(|n| n.trim().parse::<u32>().unwrap()).collect();

        Card { winning_numbers, my_numbers }
    }

    fn get_score(&self) -> u32 {
        let mut score = 0;

        for n in &self.my_numbers {
            if self.winning_numbers.contains(n) {
                score = if score == 0 { 1 } else { score * 2 };
            }
        }
        score
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

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day04.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_winning_cards_points(&contents));
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
}
