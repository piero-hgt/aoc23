use std::fs;
use text_io::scan;

#[derive(Debug)]
struct Game {
    number: u32,
    sets: Vec<GameSet>,
}

impl Game {
    // game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    fn from(game: &str) -> Game {
        // extract game id from string, then extract each color set (separated by ;)
        let number: u32;
        let sets: String;
        scan!(game.trim().bytes() => "Game {}: {}\n", number, sets);

        let mut game_sets: Vec<GameSet> = vec![];
        for set in sets.split(";") {
            game_sets.push(GameSet::from(set.trim()));
        }

        Game { number, sets: game_sets }
    }

    fn exceeds(&self, boundaries: &GameSet) -> bool {
        for gameset in &self.sets {
            if gameset.red > boundaries.red || gameset.green > boundaries.green || gameset.blue > boundaries.blue {
                return true;
            }
        }

        false
    }

    fn minimum_gameset(&self) -> GameSet {
        let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);

        for gameset in &self.sets {
            if gameset.red > red {
                red = gameset.red;
            }
            if gameset.green > green {
                green = gameset.green;
            }
            if gameset.blue > blue {
                blue = gameset.blue;
            }
        }

        GameSet { red, green, blue }
    }
}

#[derive(Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameSet {
    fn from(set: &str) -> GameSet {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for item in set.split(',') {
            let count: u32;
            let color: String;
            scan!(item.trim().bytes() => "{} {}", count, color);
            match color.as_str() {
                "red" => { red = count; },
                "green" => { green = count; },
                "blue" => { blue = count; },
                _ => panic!("invalid color : \"{}\" in item \"{}\" from set \"{}\"", color, item, set),
            }
        }
        GameSet { red, green, blue }
    }
}

fn sum_possible_game_ids(input: &str) -> u32 {
    let boundaries = GameSet { red: 12, green: 13, blue: 14 };

    let mut sum = 0;

    for line in input.lines() {
        let game = Game::from(line);
        if !game.exceeds(&boundaries) {
            sum += game.number;
        }
    }

    sum
}

fn sum_power_minimum_gameset(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let game = Game::from(line);
        let gameset = game.minimum_gameset();
        sum += gameset.red * gameset.green * gameset.blue;
    }
    sum
}

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day02.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_possible_game_ids(&contents));
}

pub fn solve_task2() {
    let contents = match fs::read_to_string("input/day02.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_power_minimum_gameset(&contents));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_possible_game_ids() {
        let text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(sum_possible_game_ids(text), 8);
    }

    #[test]
    fn test_sum_power_minimum_gameset() {
        let text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(sum_power_minimum_gameset(text), 2286);
    }
}
