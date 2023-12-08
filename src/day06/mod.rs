use std::fs;

struct Boat {
    time: u32,   // milliseconds
}

impl Boat {
    fn new(time: u32) -> Boat {
        Boat { time }
    }

    fn get_distance(&self, race_duration: u32) -> u32 {
        (race_duration - self.time) * self.time
    }
}

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Race {
        Race { time, distance }
    }

    fn count_winning_possibilities(&self) -> u32 {
        let mut count: u32 = 0;
        for time in 0..self.time {
            let boat = Boat::new(time);
            if boat.get_distance(self.time) > self.distance {
                count += 1;
            }
        }
        count
    }
}


fn sum_race_results(input: &str) -> u32 {
    let times: Vec<&str> = input.lines().nth(0).unwrap().trim().split_whitespace().collect();
    let distances: Vec<&str> = input.lines().nth(1).unwrap().trim().split_whitespace().collect();

    let mut multiply: u32 = 1;
    for i in 1..times.len() {
        let race = Race::new(times[i].parse().unwrap(), distances[i].parse().unwrap());
        let winning_possibilities = race.count_winning_possibilities();
        if winning_possibilities > 0 {
            multiply *= winning_possibilities;
        }
    }

    multiply
}


pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day06.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_race_results(&contents));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_race_results() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        assert_eq!(sum_race_results(input), 288);
    }
}
