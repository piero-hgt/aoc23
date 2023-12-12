use text_io::scan;
use ctf_brute::ops::Pattern;
use rayon::prelude::*;
use std::fs;

#[derive(Debug)]
struct Row {
    springs: String,
    damaged_groups_size: Vec<u32>
}

impl Row {
    fn new(input: &str) -> Row {
        let mut s = String::new();
        let mut d = String::new();
        scan!(input.trim().bytes() => "{} {}\n", s, d);

        let mut damaged_groups_size = Vec::new();
        for n in d.split(',') {
            if n != "" {
                damaged_groups_size.push(n.parse::<u32>().unwrap());
            }
        }

        Row {
            springs: s,
            damaged_groups_size
        }
    }

    fn count_possibilities(&self) -> u32 {
        let mut possibilities = 0;

        // Brute force all possibilities : replace all ? and check validity
        let count_unknown = self.springs.chars().filter(|s| *s == '?').count();

        // let source_pattern = format!(r"\?{{{}}}", count_unknown);
        let pattern = format!(r"[.#]{{{}}}", count_unknown);
        let pattern = Pattern::from_pattern(&pattern).expect("Failed to parse pattern");

        for result in pattern.iter() {
            let mut springs = self.springs.clone();
            // let re = Regex::new(&source_pattern).unwrap();
            for c in result.chars() {
                springs = springs.replacen('?', &c.to_string(), 1);
            }
            // println!("{}: {}", &result, &springs);
            if self.springs_match_predicates(springs.to_string()) {
                // println!("{} matches predicate", &springs);
                possibilities += 1;
            }
        }

        possibilities
    }

    fn springs_match_predicates(&self, springs: String) -> bool {
        let mut groups: Vec<u32> = Vec::new();
        let mut in_group = false;
        for c in springs.chars() {
            if c == '#' {
                if !in_group {
                    in_group = true;
                    groups.push(1);
                } else {
                    let len = groups.len();
                    groups[len - 1] += 1;
                }
            } else if c != '#' && in_group {
                in_group = false;
            }
        }
        if groups.len() != self.damaged_groups_size.len() {
            return false;
        }
        // println!("\"{}\" has {} groups of damaged springs", springs, &groups.len());
        for (i, count) in groups.iter().enumerate() {
            if *count != self.damaged_groups_size[i] {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct SpringMap {
    rows: Vec<Row>
}

impl SpringMap {
    fn new(input: &str) -> Self {
        SpringMap { rows: input.lines().map(|l| Row::new(l.trim())).collect() }
    }

    fn count_possibilities(&self) -> u32 {
        self.rows.par_iter().map(|r| r.count_possibilities()).sum()
    }
}

fn main() {
    // let input = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1";
    let input = fs::read_to_string("input.txt").expect("Failed to read input");

    let map = SpringMap::new(&input);
    println!("{}", map.count_possibilities());
}
