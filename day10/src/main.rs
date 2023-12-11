use std::collections::HashMap;
use std::cmp::{min, max};
use std::fs;

#[derive(Debug, Copy, Clone)]
struct PipeTile {
    tile: char,
    directions: (Direction, Direction)
}

impl PipeTile {
    fn get_direction(&self, from: Direction) -> Direction {
        if from == self.directions.0  {
            return self.directions.1.clone();
        }
        self.directions.0.clone()
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Pipe(PipeTile),
    Entrance,
    Useless
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Tile::Pipe(PipeTile { tile: c, directions: (Direction::Up, Direction::Down) }),
            '-' => Tile::Pipe(PipeTile { tile: c, directions: (Direction::Left, Direction::Right) }),
            'L' => Tile::Pipe(PipeTile { tile: c, directions: (Direction::Up, Direction::Right) }),
            'J' => Tile::Pipe(PipeTile { tile: c, directions: (Direction::Up, Direction::Left) }),
            'F' => Tile::Pipe(PipeTile { tile: c, directions: (Direction::Down, Direction::Right) }),
            '7' => Tile::Pipe(PipeTile { tile: c, directions: (Direction::Down, Direction::Left) }),
            'S' => Tile::Entrance,
            _ => Tile::Useless
        }
    }

    fn get_directions(&self) -> Vec<Direction> {
        match self {
            Tile::Pipe(pipe) => vec![pipe.directions.0, pipe.directions.1],
            Tile::Entrance => vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            _ => vec![]
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    map: HashMap<(usize, usize), Tile>,
    entrance: (usize, usize),
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map: HashMap<(usize, usize), Tile> = HashMap::new();
        let mut entrance: (usize, usize) = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let tile = Tile::from(c);
                match tile {
                    Tile::Entrance => entrance = (x, y),
                    _ => ()
                }
                map.insert((x, y), tile);
            }
        }

        Map { map, entrance }
    }

    fn proceed(&self) -> Vec<(usize, usize)> {
        let mut pipe: Vec<(usize, usize)> = Vec::new();
        let mut current = self.entrance;
        let mut previous = self.entrance;
        let mut direction: Option<Direction> = None;


        loop {
            let tile_move = self.find_next_tile(current, previous, direction);
            println!("next_tiles: {:?}", tile_move);

            if tile_move.to == self.entrance {
                break;
            }

            // Determine direction
            previous = current;
            current = tile_move.to.clone();
            direction = Some(tile_move.direction);
            pipe.push(current.clone());
        }
        pipe
    }

    fn get_tile_pos(&self, direction: Direction, current: (usize, usize)) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => {
                if current.1 == 0 {
                    return None;
                }
                Some((current.0, current.1 - 1))
            },
            Direction::Down => Some((current.0, current.1 + 1)),
            Direction::Left => {
                if current.0 == 0 {
                    return None;
                }
                Some((current.0 - 1, current.1))
            },
            Direction::Right => Some((current.0 + 1, current.1)),
        }
    }

    fn get_tile(&self, direction: Direction, current: (usize, usize)) -> Option<&Tile> {
        match self.get_tile_pos(direction, current) {
            Some(position) => {
                self.map.get(&position)
            },
            None => None
        }
    }

    fn find_next_tile(
        &self,
        current: (usize, usize),
        previous: (usize, usize),
        direction: Option<Direction>
    ) -> Move {
        // println!("find_next_tile current: {:?}, previous: {:?}, direction: {:?}", current, previous, direction);
        let mut directions: Vec<(Direction, (usize, usize), &Tile)> = Vec::new();

        let tile = self.map[&current];
        for d in tile.get_directions() {
            if let Some(tile_position) = self.get_tile_pos(d, current) {
                let tile = self.map.get(&tile_position).unwrap();
                match tile {
                    Tile::Useless => (),
                    _ => {
                        directions.push((d, tile_position, tile));
                    },
                }
            }
        }

        for (d, (x, y), tile) in directions {
            if previous == (x, y) {
                continue;
            }
            return Move { from: current, to: (x, y), direction: d }
        }
        unreachable!();
    }

    fn find_farthest_tile_steps(&self) -> usize {
        let mut steps: usize = 0;
        let pipe = self.proceed();
        for (i, _tile) in pipe.iter().enumerate() {
            let value = min(i + 1, pipe.len() - i);
            println!("tile: {:?} value: {}", _tile, value);
            steps = max(steps, value);
        }
        steps
    }
}

fn main() {
    // let input = "..F7.
    //     .FJ|.
    //     SJ.L7
    //     |F--J
    //     LJ...";

    // let map = Map::new(input);
    // println!("entrance: {:?}", &map.entrance);
    // println!("find_farthest_tile_steps: {}", map.find_farthest_tile_steps());

    let input = fs::read_to_string("input.txt").unwrap();
    let map = Map::new(&input);
    println!("length {}", map.find_farthest_tile_steps());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop() {
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";

        let map = Map::new(input);
        assert_eq!(map.entrance, (1, 1));
    }

    #[test]
    fn test_loop2() {
        let input = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

        let map = Map::new(input);
        assert_eq!(map.entrance, (0, 2));
    }
}
