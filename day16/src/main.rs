use std::collections::BTreeMap;

type Coordinates = [usize; 2];

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Map {
    items: BTreeMap<Coordinates, char>,
    beams: BTreeMap<Coordinates, Vec<Direction>>,
    energized: BTreeMap<Coordinates, Vec<Direction>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut items = BTreeMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                items.insert([x, y], c);
            }
        }
        Self { items, beams: BTreeMap::new(), energized: BTreeMap::new() }
    }

    fn walk(&mut self) {
        self.beams.insert([0, 0], vec![Direction::Right]);
        self.light_in([0, 0], Direction::Right);
    }

    fn light_in(&mut self, coordinates: Coordinates, direction: Direction) {
        // println!("light in : going {:?} to ({}, {})", direction, coordinates[0], coordinates[1]);
        self.energize(&coordinates, &direction);

        if let Some(c) = self.items.clone().get(&coordinates) {
            match c {
                '.' => {
                    if let Some(new) = self.new_coordinates(coordinates, &direction) {
                        self.light_in(new, direction);
                    }
                },
                '/' | '\\' => {
                    let new_direction = Self::new_direction(*c, direction);
                    if let Some(new) = self.new_coordinates(coordinates, &new_direction) {
                        self.light_in(new, new_direction);
                    }
                },
                '|' | '-' => {
                    for d in Self::split_direction(*c, direction) {
                        if let Some(new) = self.new_coordinates(coordinates, &d) {
                            match self.beams.get_mut(&new) {
                                Some(b) => {
                                    // dbg!(&b, &d);
                                    if b.contains(&d) {
                                        return;
                                    }
                                    b.push(d.clone());
                                },
                                None => {
                                    self.beams.insert(new, vec![d.clone()]);
                                }
                            }
                            self.light_in(new, d);
                        }
                    }
                },
                _ => {}
            }
        }
    }

    fn new_coordinates(&self, coordinates: Coordinates, direction: &Direction) -> Option<Coordinates> {
        let max = self.items.iter().nth_back(0).unwrap().0;
        match direction {
            Direction::Up => if coordinates[1] == 0 { None } else { Some([coordinates[0], coordinates[1] - 1]) },
            Direction::Down => if coordinates[1] == max[1] { None } else { Some([coordinates[0], coordinates[1] + 1]) },
            Direction::Right => if coordinates[0] == max[0] { None } else { Some([coordinates[0] + 1, coordinates[1]]) },
            Direction::Left => if coordinates[0] == 0 { None } else { Some([coordinates[0] - 1, coordinates[1]]) },
        }
    }

    fn new_direction(c: char, direction: Direction) -> Direction {
        match c {
            '/' => {
                match direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                }
            },
            '\\' => {
                match direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                }
            },
            _ => direction,
        }
    }

    fn split_direction(c: char, direction: Direction) -> Vec<Direction> {
        match c {
            '|' => {
                if direction == Direction::Up || direction == Direction::Down {
                    return vec![direction];
                }
                return vec![Direction::Up, Direction::Down];
            },
            '-' => {
                if direction == Direction::Right || direction == Direction::Left {
                    return vec![direction];
                }
                return vec![Direction::Right, Direction::Left];
            },
            _ => return vec![]
        }
    }

    fn energize(&mut self, coordinates: &Coordinates, direction: &Direction) {
        if let Some(v) = self.energized.get_mut(coordinates) {
            if v.contains(&direction) {
                println!("already energized : ({}, {})", coordinates[0], coordinates[1]);
            }
        } else {
            self.energized.insert(*coordinates, vec![*direction]);
        }
    }
}

fn task1() {
    // let input = ".|...\\....
    // |.-.\\.....
    // .....|-...
    // ........|.
    // ..........
    // .........\\
    // ..../.\\\\..
    // .-.-/..|..
    // .|....-|.\\
    // ..//.|....";
    let input = include_str!("../input.txt");

    let mut map = Map::new(&input);
    // dbg!(&map.items);
    map.walk();
    println!("{}", &map.energized.iter().count());
}


fn main() {
    task1();
}
