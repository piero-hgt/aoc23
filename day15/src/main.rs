
fn hash_string(s: String) -> usize {
    let mut value: usize = 0;
    for c in s.chars() {
        hash_char(c, &mut value);
    }
    value
}

fn hash_char(c: char, value: &mut usize) {
    if c == '\n' {
        return;
    }
    *value = ((*value + (c as usize))*17)%256;
}

fn main() {
    // let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let input = include_str!("../input.txt");

    let mut sum: usize = input.split(",").collect::<Vec<_>>().iter().map(|s| hash_string(s.to_string())).sum();

    println!("{}", sum);
}
