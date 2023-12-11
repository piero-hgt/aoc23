use std::fs;

struct Boat {
    time: u64,   // milliseconds
}

impl Boat {
    fn new(time: u64) -> Boat {
        Boat { time }
    }

    fn get_distance(&self, race_duration: u64) -> u64 {
        (race_duration - self.time) * self.time
    }
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        Race { time, distance }
    }

    fn count_winning_possibilities(&self) -> u64 {
        let mut count: u64 = 0;
        for time in 0..self.time {
            let boat = Boat::new(time);
            if boat.get_distance(self.time) > self.distance {
                count += 1;
            }
        }
        count
    }
}


fn sum_race_results(input: &str) -> u64 {
    let times: Vec<&str> = input.lines().nth(0).unwrap().trim().split_whitespace().collect();
    let distances: Vec<&str> = input.lines().nth(1).unwrap().trim().split_whitespace().collect();

    let mut multiply: u64 = 1;
    for i in 1..times.len() {
        let race = Race::new(times[i].parse().unwrap(), distances[i].parse().unwrap());
        let winning_possibilities = race.count_winning_possibilities();
        if winning_possibilities > 0 {
            multiply *= winning_possibilities;
        }
    }

    multiply
}

fn sum_race_results_task2(input: &str) -> u64 {
    let times: Vec<&str> = input.lines().nth(0).unwrap().trim().split_whitespace().collect();
    let distances: Vec<&str> = input.lines().nth(1).unwrap().trim().split_whitespace().collect();

    let mut time: String = String::new();
    let mut distance: String = String::new();

    for i in 1..times.len() {
        time.push_str(times[i]);
        distance.push_str(distances[i]);
    }

    println!("Time: {}, Distance: {}", time, distance);

    let race = Race::new(time.parse().unwrap(), distance.parse().unwrap());
    race.count_winning_possibilities()
}

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day06.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_race_results(&contents));
}


pub fn solve_task2() {
    let contents = match fs::read_to_string("input/day06.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_race_results_task2(&contents));
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

    #[test]
    fn test_sum_race_results_task2() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        assert_eq!(sum_race_results_task2(input), 71503);
    }
}
