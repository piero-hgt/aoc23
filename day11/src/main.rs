use std::fs;
use std::collections::BTreeMap;

type Coordinates = [usize; 2];

#[derive(Debug)]
struct Map {
    map: BTreeMap<Coordinates, char>,
    galaxies: Vec<Coordinates>,
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
        let mut map = Map { map, galaxies };
        map.expand();
        map
    }

    fn expand(&mut self) {
        self.expand_rows();
        self.expand_cols();
    }

    fn expand_rows(&mut self) {
        let (max, _) = self.map.iter().nth_back(0).unwrap();
        let mut expanded: BTreeMap<Coordinates, char> = BTreeMap::new();
        let mut new_rows = 0;

        // For each row, if only '.' => duplicate row
        for row in 0..=max[0] {
            let mut has_galaxies = false;
            for col in 0..=max[1] {
                let coordinates: Coordinates = [row + new_rows, col];
                expanded.insert(coordinates, self.map[&[row, col]]);
                if self.galaxies.contains(&[row, col]) {
                    has_galaxies = true;
                }
            }
            if !has_galaxies {
                new_rows += 1;
                for col in 0..=max[1] {
                    let coordinates: Coordinates = [row + new_rows, col];
                    expanded.insert(coordinates, '.');
                }
            }
        }

        let max = [max[0]+new_rows, max[1]];

        // update galaxies
        self.galaxies = Vec::new();
        for row in 0..=max[0] {
            for col in 0..=max[1] {
                if expanded[&[row, col]] == '#' {
                    self.galaxies.push([row, col]);
                }
            }
        }

        self.map = expanded.clone();
    }

    fn expand_cols(&mut self) {
        let (max, _) = self.map.iter().nth_back(0).unwrap();
        let mut expanded: BTreeMap<Coordinates, char> = BTreeMap::new();
        let mut new_cols = 0;

        // For each col, if only '.' => duplicate col
        for col in 0..=max[1] {
            let mut has_galaxies = false;
            for row in 0..=max[0] {
                let coordinates: Coordinates = [row, col+new_cols];
                expanded.insert(coordinates, self.map[&[row, col]]);
                if self.galaxies.contains(&[row, col]) {
                    has_galaxies = true;
                }
            }
            if !has_galaxies {
                new_cols += 1;
                for row in 0..=max[0] {
                    let coordinates: Coordinates = [row, col+new_cols];
                    expanded.insert(coordinates, '.');
                }
            }
        }

        let max = [max[0], max[1]+new_cols];
        // New galaxies
        self.galaxies = Vec::new();
        for row in 0..=max[0] {
            for col in 0..=max[1] {
                if expanded[&[row, col]] == '#' {
                    self.galaxies.push([row, col]);
                }
            }
        }

        self.map = expanded.clone();
    }

    fn print_expanded(&self) {
        let (max, _) = self.map.iter().nth_back(0).unwrap();
        let mut galaxy = 1;
        for row in 0..=max[0] {
            for col in 0..=max[1] {
                if self.galaxies.contains(&[row, col]) {
                    print!("{}", galaxy);
                    galaxy += 1;
                } else {
                    print!("{}", self.map[&[row, col]]);
                }
            }
            println!();
        }
    }

    fn find_distance_between(&self, a: usize, b: usize) -> usize {
        let a = self.galaxies[a];
        let b = self.galaxies[b];

        let x = if a[0] < b[0] {
            b[0] - a[0]
        } else {
            a[0] - b[0]
        };

        let y = if a[1] < b[1] {
            b[1] - a[1]
        } else {
            a[1] - b[1]
        };

        x + y
    }
}

fn main() {
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
    let input = fs::read_to_string("input.txt").unwrap();

    let map = Map::new(&input);
    map.print_expanded();

    let mut sum = 0;
    for a in 0..map.galaxies.len()-1 {
        for b in a+1..map.galaxies.len() {
            let distance = map.find_distance_between(a, b) as u32;
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
