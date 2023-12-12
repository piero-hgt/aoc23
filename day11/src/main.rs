use std::fs;
use std::collections::BTreeMap;
use std::cmp::{min, max};

type Coordinates = [usize; 2];

#[derive(Debug)]
struct Map {
    map: BTreeMap<Coordinates, char>,
    galaxies: Vec<Coordinates>,
    empty_rows: Vec<u32>,
    empty_cols: Vec<u32>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = BTreeMap::new();
        let mut galaxies = Vec::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    galaxies.push([row, col]);
                }
                map.insert([row, col], c);
            }
        }

        let (max, _) = map.iter().nth_back(0).unwrap();
        let mut empty_rows: Vec<u32> = Vec::new();
        for row in 0..=max[0] {
            let mut has_galaxies = false;
            for col in 0..=max[1] {
                if galaxies.contains(&[row, col]) {
                    has_galaxies = true;
                    break
                }
            }
            if !has_galaxies {
                empty_rows.push(row as u32);
            }
        }
        let mut empty_cols: Vec<u32> = Vec::new();
        for col in 0..=max[1] {
            let mut has_galaxies = false;
            for row in 0..=max[0] {
                if galaxies.contains(&[row, col]) {
                    has_galaxies = true;
                    break;
                }
            }
            if !has_galaxies {
                empty_cols.push(col as u32);
            }
        }

        Map { map, galaxies, empty_rows, empty_cols }
    }

    fn find_distance_between(&self, a: usize, b: usize, expand: usize) -> usize {
        let a = self.galaxies[a];
        let b = self.galaxies[b];

        let min_row = min(a[0], b[0]);
        let max_row = max(a[0], b[0]);
        let min_col = min(a[1], b[1]);
        let max_col = max(a[1], b[1]);

        let mut distance: usize = 0;
        let mut empty_rows: usize = 0;
        for row in (min_row+1)..=max_row {
            if self.empty_rows.contains(&(row as u32)) {
                empty_rows += 1;
            } else {
                distance += 1;
            }
        }
        distance += empty_rows * expand;

        let mut empty_cols = 0;
        for col in (min_col+1)..=max_col {
            if self.empty_cols.contains(&(col as u32)) {
                empty_cols += 1;
            } else {
                distance += 1;
            }
        }
        distance += empty_cols * expand;
        distance
    }
}

fn main() {
    task2();
}

fn task1() {
    // let input = fs::read_to_string("input.txt").unwrap();
    let input =
   "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    let mut map = Map::new(&input);

    // println!("distance between 1 and 2 with expand=1 : {}", map.find_distance_between(0, 1, 1));
    let mut sum = 0;
    for a in 0..map.galaxies.len()-1 {
        for b in a+1..map.galaxies.len() {
            let distance = map.find_distance_between(a, b, 1) as u32;
            sum += distance;
            // println!("{} to {} : {}", a+1, b+1, distance);
        }
    }
    println!("Sum: {}", sum);
}

fn task2() {
    let input = fs::read_to_string("input.txt").unwrap();
//     let input =
//    "...#......
//     .......#..
//     #.........
//     ..........
//     ......#...
//     .#........
//     .........#
//     ..........
//     .......#..
//     #...#.....";

    let mut map = Map::new(&input);

    let mut sum: usize = 0;
    for a in 0..map.galaxies.len()-1 {
        for b in a+1..map.galaxies.len() {
            let distance = map.find_distance_between(a, b, 1_000_000);
            sum += distance;
            // println!("{} to {} : {}", a+1, b+1, distance);
        }
    }
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        println!("{:?}", Map::new(input));
    }
}
