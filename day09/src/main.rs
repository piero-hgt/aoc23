use std::fs;

#[derive(Debug)]
struct History {
    sequence: Vec<i64>,
    next: Option<Box<History>>
}

impl History  {
    fn new(input: &str) -> History {
        let sequence: Vec<i64> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

        let mut dedup_sequence: Vec<i64> = sequence.clone();
        dedup_sequence.dedup();
        if dedup_sequence.len() == 1 && dedup_sequence[0] == 0 {
            return History {
                sequence,
                next: None
            }
        }

        let mut next_input: Vec<i64> = Vec::new();

        for i in 0..sequence.len()-1 {
            next_input.push(sequence[i+1] - sequence[i]);
        }

        let next_input: String = next_input.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");
        History {
            sequence,
            next: Some(Box::new(History::new(&next_input)))
        }
    }

    fn get_next_history_value(&self) -> i64 {
        if self.next.is_none() {
            return self.sequence[self.sequence.len() - 1];
        }
        let step = self.next.as_ref().unwrap().get_next_history_value();
        self.sequence[self.sequence.len() - 1] + step
    }
}

fn get_next_history_values(input: &str) -> i64 {
    let histories: Vec<History> = input.lines().map(|s| History::new(s)).collect();
    histories.iter().map(|h| h.get_next_history_value()).sum::<i64>()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", get_next_history_values(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        assert_eq!(get_next_history_values(input), 114);
    }

    #[test]
    fn test_history_neg() {
        let input = "0 -3 -6 -9 -12 -15";

        assert_eq!(get_next_history_values(input), -18);
    }
}
