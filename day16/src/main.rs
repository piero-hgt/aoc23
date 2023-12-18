use std::collections::{BTreeMap, BinaryHeap};
use rayon::prelude::*;

type Coordinates = [usize; 2];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum CellType {
    Dot,
    Pipe,
    Dash,
    Slash,
    BackSlash
}

impl CellType {
    fn try_from_char(c: char) -> CellType {
        match c {
            '.' => CellType::Dot,
            '|' => CellType::Pipe,
            '-' => CellType::Dash,
            '/' => CellType::Slash,
            '\\' => CellType::BackSlash,
            _ => unreachable!("unknown char"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct QueueItem {
    coordinates: Coordinates,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Cell {
    c: CellType,
    beams: Vec<Direction>,
}

#[derive(Debug, Clone)]
struct Map {
    items: BTreeMap<Coordinates, Cell>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut items = BTreeMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                items.insert([x, y], Cell { c: CellType::try_from_char(c), beams: vec![] });
            }
        }
        Self { items }
    }

    fn walk(&mut self, coordinates: Coordinates, direction: Direction) {
        let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();

        queue.push(QueueItem { coordinates, direction });
        while let Some(item) = queue.pop() {
            let (coordinates, direction) = (item.coordinates, item.direction);
            // println!("({}, {}) -> {:?}", coordinates[0], coordinates[1], direction);

            if self.already_energized(&coordinates, &direction) {
                // println!("already energized ({}, {}) -> {:?}", coordinates[0], coordinates[1], direction);
                continue;
            }
            self.energize(&coordinates, &direction);

            if let Some(cell) = self.items.get(&coordinates) {
                match cell.c {
                    CellType::Pipe | CellType::Dash => {
                        for d in Self::split_direction(cell.c, direction) {
                            if let Some(new) = self.new_coordinates(coordinates, &d) {
                                if let Some(new_cell) = self.items.get(&new) {
                                    if new_cell.beams.contains(&d) {
                                        continue;
                                    }
                                }
                                queue.push(QueueItem { coordinates: new, direction: d });
                            }
                        }
                    },
                    _ => {
                        let new_direction = Self::new_direction(cell.c, direction);
                        if let Some(new) = self.new_coordinates(coordinates, &new_direction) {
                            queue.push(QueueItem { coordinates: new, direction: new_direction });
                        }
                    }
                }
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

    fn new_direction(c: CellType, direction: Direction) -> Direction {
        match c {
            CellType::Slash  => {
                match direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                }
            },
            CellType::BackSlash => {
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

    fn split_direction(c: CellType, direction: Direction) -> Vec<Direction> {
        match c {
            CellType::Pipe => {
                if direction == Direction::Up || direction == Direction::Down {
                    return vec![direction];
                }
                return vec![Direction::Up, Direction::Down];
            },
            CellType::Dash => {
                if direction == Direction::Right || direction == Direction::Left {
                    return vec![direction];
                }
                return vec![Direction::Right, Direction::Left];
            },
            _ => return vec![]
        }
    }

    fn energize(&mut self, coordinates: &Coordinates, direction: &Direction) {
        if let Some(cell) = self.items.get_mut(coordinates) {
            cell.beams.push(direction.clone());
        }
    }

    fn already_energized(&self, coordinates: &Coordinates, direction: &Direction) -> bool {
        if let Some(cell) = self.items.get(coordinates) {
            cell.beams.contains(&direction)
        } else {
            false
        }
    }

    fn get_energized(&self) -> usize {
        self.items.iter().filter(|(_, c)| c.beams.len() > 0).count()
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
    map.walk([0, 0], Direction::Right);
    println!("{}", &map.get_energized());
}

fn task2() {
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

    let map = Map::new(&input);

    let run_it = |c: Coordinates, d: Direction| -> usize {
        let mut cloned = map.clone();
        println!("Walk from ({}, {}) -> {:?}", c[0], c[1], d);
        cloned.walk(c, d);

        let current = cloned.get_energized();
        println!("({}, {}) -> {:?} : {}", c[0], c[1], d, current);

        current
    };

    let height = map.items.keys().map(|c| c[0]).max().unwrap();
    let width = map.items.keys().map(|c| c[1]).max().unwrap();

    let max_v = (0..=width).into_par_iter().map(|x| {
        let down = run_it([x, 0], Direction::Down);
        let up = run_it([x, height], Direction::Up);
        down.max(up)
    }).max();

    let max_h = (0..=height).into_par_iter().map(|y| {
        let right = run_it([0, y], Direction::Right);
        let left = run_it([width, y], Direction::Left);
        right.max(left)
    }).max();

    println!("best is {}", max_v.max(max_h).unwrap());
}

fn main() {
    task2();
}
