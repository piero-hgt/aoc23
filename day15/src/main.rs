
#[derive(Debug)]
struct Instruction {
    label: String,
    operation: char,
    box_number: usize,
    lens: Option<usize>,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let (label, value) = s.trim().split_once(|c| matches!(c, '-' | '=')).unwrap();
        let operation = s.chars().find(|c| matches!(*c, '-' | '=')).unwrap();
        let lens = (operation == '=').then(|| value.parse::<usize>().unwrap());

        Instruction {
            label: label.to_string(),
            operation,
            box_number: hash_string(label),
            lens
        }
    }
}

#[derive(Debug)]
struct LightBox {
    slots: Vec<Slot>,
}

impl LightBox {
    fn add_lens(&mut self, label: String, lens: usize) {
        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.label == label) {
            slot.lens = lens;
        } else {
            self.slots.push(Slot { label, lens });
        }
    }

    fn remove_lens(&mut self, label: String) {
        if let Some(position) = self.slots.iter().position(|slot| slot.label == label) {
            self.slots.remove(position);
        }
    }

    fn get_focusing_power(&self, box_number: usize) -> usize {
        self.slots.iter().enumerate().map(|(i, slot)| (box_number + 1) * (i + 1) * slot.lens).sum()
    }
}

#[derive(Debug)]
struct Slot {
    label: String,
    lens: usize
}

fn hash_string(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize)*17)%256)
}

fn task1() {
    // let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let input = include_str!("../input.txt");
    let sum: usize = input.split(",").map(|s| hash_string(s.trim())).sum();
    println!("{}", sum);
}

fn task2() {
    // let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let input = include_str!("../input.txt");

    let instructions: Vec<Instruction> = input.split(",").map(Instruction::new).collect();

    let mut boxes: Vec<LightBox> = (0..256).map(|_| LightBox { slots: vec![] }).collect();

    for i in instructions {
        match i.operation {
            '=' => boxes[i.box_number].add_lens(i.label, i.lens.unwrap()),
            '-' => boxes[i.box_number].remove_lens(i.label),
            _ => unreachable!()
        }
    }

    let sum: usize = (0..boxes.len()).map(|i| boxes[i].get_focusing_power(i)).sum();
    println!("{}", sum);
}

fn main() {
    task2();
}
