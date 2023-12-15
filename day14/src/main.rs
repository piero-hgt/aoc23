#[derive(Debug)]
enum Direction {
    North,
    // South,
    // East,
    // West,
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<char>>,
}

impl Pattern {
    fn new(input: &str) -> Self {
        let rows: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect()).collect();
        Pattern { rows }
    }

    fn print(&self) {
        for row in &self.rows {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }

    fn tilt(&mut self, direction: Direction) {
        let start = match direction {
            Direction::North => 1,
            // Direction::South => self.rows.len() - 2,
            // Direction::East => 0,
            // Direction::West => self.rows[0].len() - 1,
        };

        let mut changed = false;
        for r in start..self.rows.len() {
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
            self.tilt(direction);
        }
    }

    fn sum_weight(&self) -> usize {
        let mut sum: usize = 0;
        for (i, row) in self.rows.iter().enumerate() {
            let row_weight = (row.iter().filter(|&&c| c == 'O').count()) * (self.rows.len() - i);
            println!("line {} has a weight of {}", i, row_weight);
            sum += row_weight;
        }
        sum
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
    pattern.tilt(Direction::North);
    pattern.print();

    println!("{}", pattern.sum_weight());
}


fn task2() {

}
