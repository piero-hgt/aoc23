
type Reflection = [usize; 2];

#[derive(Debug)]
enum ReflectionType {
    Horizontal(Reflection),
    Vertical(Reflection),
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

    fn is_reflection_line(i: usize, r: &Vec<Vec<char>>) -> bool {
        (0..i.min(r.len() - i)).all(|idx| r[i - idx - 1] == r[i + idx])
    }
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(|p| Pattern::new(p)).collect()
}

fn main() {

}

fn task1() {
        // let input = "#.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#.

    // #...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#";
    let input = include_str!("../input.txt");

    let patterns = parse_patterns(&input);
    let mut sum = 0;
    for (i, p) in patterns.iter().enumerate() {
        let h = (1..p.rows.len())
            .skip_while(|&i| !Pattern::is_reflection_line(i, &p.rows))
            .next()
            .unwrap_or_default();
        let v = (1..p.cols.len())
            .skip_while(|&i| !Pattern::is_reflection_line(i, &p.cols))
            .next()
            .unwrap_or_default();

        sum += v + 100*h;
    }
    println!("{}", sum);
}
