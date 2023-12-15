#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

impl Pattern {
    fn new(input: &str) -> Self {
        let rows: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect()).collect();
        let cols: Vec<Vec<char>> = (0..rows[0].len()).map(|i| rows.iter().map(|row| row[i]).collect()).collect();
        Pattern { rows, cols }
    }

    fn print(&self) {
        for row in &self.rows {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }

    fn tilt_cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn tilt(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.tilt_north(),
            Direction::South => self.tilt_south(),
            Direction::East => self.tilt_east(),
            Direction::West => self.tilt_west()
        };
    }

    fn tilt_north(&mut self) {
        // println!("Tilt North");
        let mut changed = false;
        for r in 1..self.rows.len() {
            for c in 0..self.rows[r].len() {
                if self.rows[r][c] != 'O' {
                    continue;
                }
                if self.rows[r-1][c] == '.' {
                    changed = true;
                    self.rows[r-1][c] = self.rows[r][c];
                    self.rows[r][c] = '.';
                }
            }
        }

        if changed {
            self.tilt_north();
        }

        self.cols = (0..self.rows[0].len()).map(|i| self.rows.iter().map(|row| row[i]).collect()).collect();
    }

    fn tilt_south(&mut self) {
        // println!("Tilt South");
        let mut changed = false;
        for r in (0..self.rows.len()-1).rev() {
            for c in 0..self.rows[r].len() {
                if self.rows[r][c] != 'O' {
                    continue;
                }
                if self.rows[r+1][c] == '.' {
                    changed = true;
                    self.rows[r+1][c] = self.rows[r][c];
                    self.rows[r][c] = '.';
                }
            }
        }

        if changed {
            self.tilt_south();
        }

        self.cols = (0..self.rows[0].len()).map(|i| self.rows.iter().map(|row| row[i]).collect()).collect();
    }

    fn tilt_west(&mut self) {
        // println!("Tilt West");
        let mut changed = false;
        for r in 1..self.cols.len() {
            for c in 0..self.cols[r].len() {
                if self.cols[r][c] != 'O' {
                    continue;
                }
                if self.cols[r-1][c] == '.' {
                    changed = true;
                    self.cols[r-1][c] = self.cols[r][c];
                    self.cols[r][c] = '.';
                }
            }
        }

        if changed {
            self.tilt_west();
        }

        self.rows = (0..self.cols[0].len()).map(|i| self.cols.iter().map(|col| col[i]).collect()).collect();
    }

    fn tilt_east(&mut self) {
        // println!("Tilt East");
        let mut changed = false;
        for r in (0..self.cols.len()-1).rev() {
            for c in 0..self.cols[r].len() {
                if self.cols[r][c] != 'O' {
                    continue;
                }
                if self.cols[r+1][c] == '.' {
                    changed = true;
                    self.cols[r+1][c] = self.cols[r][c];
                    self.cols[r][c] = '.';
                }
            }
        }

        if changed {
            self.tilt_east();
        }

        self.rows = (0..self.cols[0].len()).map(|i| self.cols.iter().map(|col| col[i]).collect()).collect();
    }

    fn sum_weight(&self) -> usize {
        self.rows.iter().enumerate().map(|(i, row)| (row.iter().filter(|&&c| c == 'O').count()) * (self.rows.len() - i)).sum()
    }

}

fn main() {
    task1();
}

fn task1() {
    // let input = "O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....";
    let input = include_str!("../input.txt");

    let mut pattern = Pattern::new(&input);
    pattern.print();
    for i in 0..1000 {
        pattern.tilt_cycle();
        println!("{:04}: {}", i+1, pattern.sum_weight());
    }
    pattern.print();

    println!("{}", pattern.sum_weight());
}


fn task2() {

}
