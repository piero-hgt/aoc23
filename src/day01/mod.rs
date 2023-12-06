use std::fs;

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day01task1.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_calibration(&contents));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let calibration = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(sum_calibration(calibration), 142);
    }
}
