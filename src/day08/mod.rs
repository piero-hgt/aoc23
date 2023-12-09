use std::collections::HashMap;
use std::fs;
use text_io::scan;

#[derive(Debug)]
struct ElfMap {
    directions: Vec<char>,
    instructions: HashMap<String, (String, String)>
}

impl ElfMap {
    fn new(input: &str) -> Self {
        let directions: Vec<char> = input.lines().nth(0).unwrap().chars().collect();

        let mut instructions = HashMap::new();

        for line in input.lines().skip(2) {
            let (from, left, right): (String, String, String);
            scan!(line.trim().bytes() => "{} = ({}, {})", from, left, right);

            instructions.insert(from, (left, right));
        }

        ElfMap { directions, instructions }
    }

    fn goto(&self, from: &str, to: &str) -> u32 {
        let mut from = String::from(from);
        let mut count: u32 = 0;

        while from != to {
            for c in self.directions.iter() {
                let direction = self.instructions.get(&from).unwrap();

                from = match c {
                    'L' => direction.0.clone(),
                    'R' => direction.1.clone(),
                    _ => panic!("Invalid direction")
                };

                count += 1;

                if from == to {
                    break;
                }
            }
        }
        count
    }

    fn ghosts_goto(&self) -> u32 {
        let mut start: Vec<String> = self.instructions
            .keys()
            .filter(|&k| k.chars().nth(2).unwrap() == 'A')
            .map(|v| v.clone())
            .collect();

        let mut count: u32 = 0;

        let mut finished: bool = false;

        while finished == false {
            for c in self.directions.iter() {
                for from in start.iter_mut() {
                    let direction = self.instructions.get(&from.clone()).unwrap();

                    *from = match c {
                        'L' => direction.0.clone(),
                        'R' => direction.1.clone(),
                        _ => panic!("Invalid direction")
                    };

                }

                count += 1;

                println!("{}", count);

                let end: Vec<_> = start.iter().filter(|&k| k.chars().nth(2).unwrap() == 'Z').collect();
                if end.len() == start.len() {
                    finished = true;
                    break;
                }
            }
        }

        count
    }
}

fn count_steps(input: &str) -> u32 {
    let elf_map = ElfMap::new(input);

    elf_map.goto("AAA", "ZZZ")
}

fn count_ghosts_steps(input: &str) -> u32 {
    let elf_map = ElfMap::new(input);

    elf_map.ghosts_goto()
}

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day08.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", count_steps(&contents));
}

pub fn solve_task2() {
    let contents = match fs::read_to_string("input/day08.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", count_ghosts_steps(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_steps() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(count_steps(input), 6);
    }

    #[test]
    fn test_count_ghosts_steps() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        assert_eq!(count_ghosts_steps(input), 6);
    }
}
