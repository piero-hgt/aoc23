use std::fs;
use std::collections::HashMap;

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day01task1.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_calibration(&contents));
}

pub fn solve_task2() {
    let contents = match fs::read_to_string("input/day01task1.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_calibration(&convert_string_to_numbers(&contents)));
}

fn sum_calibration(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.split_whitespace() {
        let first_digit = first_digit(line);
        let last_digit = last_digit(line);

        sum += format!(
            "{}{}",
            first_digit,
            last_digit
        ).parse::<u32>().unwrap();
    }

    sum
}

fn first_digit(s: &str) -> u32 {
    for c in s.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }

    0
}

fn last_digit(s: &str) -> u32 {
    for c in s.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }

    0
}

fn convert_string_to_numbers(s: &str) -> String {
    let numbers: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut converted = String::from("");

    for (index, c) in s.chars().enumerate() {
        let mut found = false;
        for (&number, &digit) in &numbers {
            match s[index..].match_indices(number).next() {
                Some((position, _)) => {
                    if position == 0 {
                        converted.push(digit);
                        found = true;
                        break;
                    }
                },
                None => continue,
            }
        }

        if !found {
            converted.push(c);
        }
    }

    converted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_task1() {
        let calibration = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(sum_calibration(calibration), 142);
    }

    #[test]
    fn test_solve_task2() {
        let calibration = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(sum_calibration(&convert_string_to_numbers(calibration)), 281);
    }
}
