
use std::collections::*;

use regex::Regex;

lazy_static::lazy_static! {
    static ref REGEX_PAT: Regex = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();
}

fn main() {
    // Read input
    let (graph, instructions) = {
        let input = include_str!("../input/day08.txt")
            .lines()
            .map(str::trim)
            .collect::<Vec<&str>>();

        let instructions = input[0];
        // Validate input
        assert!(instructions.chars().all(|ch| ['L', 'R'].contains(&ch)));

        let mut graph = BTreeMap::new();

        for line in &input[2..] {
            let (_, [node_name, left, right]) = REGEX_PAT.captures(line).unwrap().extract();
            graph.insert(node_name, [left, right]);
        }

        (graph, instructions)
    };

    let nodes_to_process = graph.keys().copied().filter(|key| key.ends_with('A'));

    let process_node = |mut current_node: &str| {
        for step_number in 1..usize::MAX {
            for instruction in instructions.chars() {
                let direction_index = (instruction == 'R') as usize;
                let new_node = graph[current_node][direction_index];
                current_node = new_node;
            }

            if current_node.ends_with('Z') {
                return step_number;
            }
        }

        // // It is guaranteed that there's an answer for each starting node
        //
        // //  I used this code to check for loops tho:
        //
        // for (instruction_number, instruction) in instructions.chars().enumerate() {
        // let mut vis = HashSet::new();
        // if !vis.insert((instruction_number, current_node)) {
        //     panic!("Loop for node");
        // }
        unreachable!();
    };

    let amount_of_steps_for_each_node = nodes_to_process
        .map(process_node)
        .map(|x| x * instructions.len());

    let ans = amount_of_steps_for_each_node.fold(1, lcm);
    dbg!(ans);
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let tmp_a = a;
        a = b;
        b = tmp_a % b;
    }
    return a;
}
