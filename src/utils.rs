use std::fs::File;
use std::io::prelude::*;

pub fn read_puzzle_input(number: u8) -> String {
    let mut input =
        File::open(format!("./inputs/{}.txt", number)).expect("Puzzle input not found...");

    let mut text = String::new();

    input
        .read_to_string(&mut text)
        .expect("cannot read input file");

    return text;
}
