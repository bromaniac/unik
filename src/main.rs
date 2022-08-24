use itertools::Itertools;
use std::io;
use std::io::Read;

fn unique_chars(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    let mut y = v.clone();

    y.dedup();
    v.len() == y.len()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin.");

    for line in input.lines() {
        let s = line.chars().sorted().collect::<String>();
        match unique_chars(&s) {
            true => println!("{}", line),
            false => (),
        }
    }
}
