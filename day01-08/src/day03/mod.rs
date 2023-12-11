use std::collections::HashMap;
use std::fs;

struct EngineSchematic {
    dots: Vec<SchematicDot>,
    boundaries: (usize, usize)
}

impl EngineSchematic {
    fn from(input: &str) -> Self {
        let mut dots: Vec<SchematicDot> = Vec::new();
        let rows = input.lines().enumerate().count();
        let cols = input.lines().next().unwrap().trim().len();

        for (row, line) in input.lines().enumerate() {
            for (col, value) in line.trim().chars().enumerate() {
                dots.push(SchematicDot::new(row, col, value));
            }
        }

        EngineSchematic { dots, boundaries: (rows, cols) }
    }

    fn find_value_boxes(&self) -> Vec<SchematicValueBox> {
        let mut boxes: Vec<SchematicValueBox> = Vec::new();
        let mut current_box: Option<SchematicValueBox> = None;

        for dot in self.dots.iter() {
            if dot.dot_type == SchematicDotType::Number {
                match current_box {
                    Some(mut box_value) => {
                        box_value.add_value(dot.value);
                        current_box = Some(box_value);
                    },
                    None => {
                        let box_value = SchematicValueBox::new(dot.row, dot.col, dot.value);
                        current_box = Some(box_value);
                    }
                }
            }

            if dot.dot_type != SchematicDotType::Number {
                if let Some(mut box_value) = current_box {
                    boxes.push(box_value);
                    current_box = None;
                }
            }
        }

        boxes
    }

    fn find_valid_value_boxes(&self) -> Vec<SchematicValueBox> {
        let mut boxes: Vec<SchematicValueBox> = Vec::new();

        for current_box in self.find_value_boxes().iter() {
            if self.has_symbol_around(current_box) {
                boxes.push(current_box.clone());
            }
        }

        boxes
    }

    fn has_symbol_around(&self, current_box: &SchematicValueBox) -> bool {
        let (mut rs, mut cs) = current_box.box_start;
        let (mut re, mut ce) = current_box.box_end;
        let (rb, cb) = self.boundaries;

        rs -= if rs > 0 { 1 } else { 0 };
        cs -= if cs > 0 { 1 } else { 0 };
        re += if re < rb { 1 } else { 0 };
        ce += if ce < cb { 1 } else { 0 };

        for dot in self.dots.iter() {
            if dot.dot_type == SchematicDotType::Symbol && dot.is_included_in(rs, cs, re, ce) {
                return true;
            }
        }
        false
    }

    fn find_valid_gear_symbols(&self) -> Vec<Vec<SchematicValueBox>> {
        let mut vec_boxes: Vec<Vec<SchematicValueBox>> = Vec::new();

        let (rb, cb) = self.boundaries;

        for dot in self.dots.iter() {
            let mut boxes: Vec<SchematicValueBox> = Vec::new();
            if dot.value != '*' {
                continue;
            }

            println!("Found * at ({},{})", dot.row, dot.col);

            let mut current_boxes: Vec<SchematicValueBox> = Vec::new();
            for current_box in self.find_value_boxes().iter() {
                if current_box.is_adjacent_of(dot.row, dot.col) {
                    current_boxes.push(current_box.clone());
                }
            }
            if current_boxes.len() >= 2 {
                for current_box in current_boxes {
                    boxes.push(current_box.clone());
                }
            }
            if boxes.len() > 0 {
                vec_boxes.push(boxes);
            }
        }
        vec_boxes
    }
}

#[derive(Debug)]
struct SchematicDot {
    row: usize,
    col: usize,
    value: char,
    dot_type: SchematicDotType,
}

impl SchematicDot {
    fn new(row: usize, col: usize, value: char) -> Self {
        SchematicDot { row, col, value, dot_type: find_type(value) }
    }

    fn is_included_in(&self, rs: usize, cs: usize, re: usize, ce: usize) -> bool {
        self.row >= rs && self.row <= re && self.col >= cs && self.col <= ce
    }
}

#[derive(Debug, PartialEq)]
enum SchematicDotType {
    Number,
    Symbol,
    None
}

#[derive(Debug, Clone)]
struct SchematicValueBox {
    value: String,
    box_start: (usize, usize),
    box_end: (usize, usize),
}

impl SchematicValueBox {
    fn new(row: usize, col: usize, value: char) -> Self {
        SchematicValueBox {
            value: String::from(value),
            box_start: (row, col),
            box_end: (row, col)
        }
    }

    fn add_value(&mut self, value: char) {
        self.value.push(value);
        let (r, c) = self.box_end;
        self.box_end = (r, c+1);
    }

    fn is_adjacent_of(&self, row: usize, col: usize) -> bool {
        let (mut rs, mut cs) = self.box_start;
        let (mut re, mut ce) = self.box_end;

        rs -= if rs > 0 { 1 } else { 0 };
        cs -= if cs > 0 { 1 } else { 0 };
        re += 1;
        ce += 1;

        row >= rs && row <= re && col >= cs && col <= ce
    }
}

fn find_type(c: char) -> SchematicDotType {
    match c.to_digit(10) {
        Some(n) => SchematicDotType::Number,
        None => match c {
            '.' => SchematicDotType::None,
            _ => SchematicDotType::Symbol
        }
    }
}

fn sum_engine_parts(input: &str) -> u32 {
    let schematic = EngineSchematic::from(input);
    let mut sum = 0;

    for current_box in schematic.find_valid_value_boxes() {
        sum += current_box.value.parse::<u32>().unwrap();
    }
    sum
}

fn sum_gear_ratio(input: &str) -> u32 {
    let schematic = EngineSchematic::from(input);
    let mut sum = 0;

    for current_box in schematic.find_valid_gear_symbols() {
        let mut gear_mul: u32 = 1;
        for b in current_box.iter() {
            gear_mul *= b.value.parse::<u32>().unwrap();
        }
        sum += gear_mul;
    }
    sum
}

pub fn solve_task1() {
    let contents = match fs::read_to_string("input/day03.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_engine_parts(&contents));
}

pub fn solve_task2() {
    let contents = match fs::read_to_string("input/day03.txt") {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    };

    println!("{}", sum_gear_ratio(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_type() {
        assert_eq!(find_type('.'), SchematicDotType::None);
        assert_eq!(find_type('0'), SchematicDotType::Number);
        assert_eq!(find_type('*'), SchematicDotType::Symbol);
    }

    #[test]
    fn test_sum_engine_parts() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        assert_eq!(sum_engine_parts(input), 4361);
    }


    #[test]
    fn test_sum_gear_ratio() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        assert_eq!(sum_gear_ratio(input), 467835);
    }
}
