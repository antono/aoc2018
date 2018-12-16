// --- Day 2: Inventory Management System ---
// You stop falling through time, catch your breath, and check the screen on the
// device. "Destination reached. Current Year: 1518. Current Location: North
// Pole Utility Closet 83N10." You made it! Now, to find those anomalies.

// Outside the utility closet, you hear footsteps and a voice. "...I'm not sure
// either. But now that so many people have chimneys, maybe he could sneak in
// that way?" Another voice responds, "Actually, we've been working on a new
// kind of suit that would let him fit through tight spaces like that. But, I
// heard that a few days ago, they lost the prototype fabric, the design plans,
// everything! Nobody on the team can even seem to remember important details of
// the project!"

// "Wouldn't they have had enough fabric to fill several boxes in the warehouse?
// They'd be stored together, so the box IDs should be similar. Too bad it would
// take forever to search the warehouse for two similar box IDs..." They walk
// too far away to hear any more.

// Late at night, you sneak to the warehouse - who knows what kinds of paradoxes
// you could cause if you were discovered - and use your fancy wrist device to
// quickly scan every box and produce a list of the likely candidates (your
// puzzle input).

// To make sure you didn't miss any, you scan the likely candidate boxes again,
// counting the number that have an ID containing exactly two of any letter and
// then separately counting those with exactly three of any letter. You can
// multiply those two counts together to get a rudimentary checksum and compare
// it to what your device predicts.

// For example, if you see the following box IDs:

// abcdef contains no letters that appear exactly two or three times.
// bababc contains two a and three b, so it counts for both.
// abbcde contains two b, but no letter appears exactly three times.
// abcccd contains three c, but no letter appears exactly two times.
// aabcdd contains two a and two d, but it only counts once.
// abcdee contains two e.
// ababab contains three a and three b, but it only counts once.

// Of these box IDs, four of them contain a letter which appears exactly twice,
// and three of them contain a letter which appears exactly three times.
// Multiplying these together produces a checksum of 4 * 3 = 12.

// What is the checksum for your list of box IDs?

extern crate utils;
use std::collections::HashMap;

fn part_one() {
    let input = utils::read_puzzle_input(2);
    let mut seen_two_letters_count = 0;
    let mut seen_three_letters_count = 0;

    for line in input.lines() {
        let mut counter: HashMap<char, u8> = HashMap::new();

        for chr in line.chars() {
            counter
                .entry(chr)
                .and_modify(|count| { *count += 1})
                .or_insert(1);
        }

        if counter.values().find(|v| { **v == 2 }).is_some() {
            seen_two_letters_count += 1;
        }

        if counter.values().find(|v| { **v == 3 }).is_some() {
            seen_three_letters_count += 1;
        }
    }

    println!("{}", seen_three_letters_count * seen_two_letters_count)
}

// Confident that your list of box IDs is complete, you're ready to find the
// boxes full of prototype fabric.
//
// The boxes will have IDs which differ by exactly one character at the same
// position in both strings. For example, given the following box IDs:
//
// abcde
// fghij
// klmno
// pqrst
// fguij
// axcye
// wvxyz
//
// The IDs abcde and axcye are close, but they differ by two characters (the
// second and fourth). However, the IDs fghij and fguij differ by exactly one
// character, the third (h and u). Those must be the correct boxes.
//
// What letters are common between the two correct box IDs? (In the example
// above, this is found by removing the differing character from either ID,
// producing fgij.)

use std::char;

fn part_two() {
    let input = utils::read_puzzle_input(2);
    let mut results: HashMap<String, Vec<_>> = HashMap::new();
    let mut lines_count = 0;
    let mut ids_count = 0;

    for id in input.lines() {
        let ids = all_valiants_with_1_letter_replaced(id);

        lines_count += 1;

        for id_changed in ids {
            ids_count += 1;
            results.entry(id_changed)
                .and_modify(|vec| vec.push(id.clone()))
                .or_insert(vec!(id));
        }
    }

    let mut res = vec!();

    for (matcher, ids) in &results {
        if ids.len() > 1 {
            res.push((matcher.clone(), ids.clone()));
        }
    }

    println!("{}", "-".repeat(100));

    for tup in &res {
        println!("{:?}", tup);
    }

    println!("{}", "-".repeat(100));

    println!("Res.len: {:?}", res.len());
    println!("Keys: {}", results.keys().len());
    println!("Values: {}", results.values().fold(0, |acc, arr| acc + arr.len()));
    println!("Lines: {}, ids: {}", lines_count, ids_count);
}


fn all_valiants_with_1_letter_replaced(id: &str) -> Vec<String> {
    let mut result: Vec<String> = vec!();
    let id_map = id.chars().collect::<Vec<char>>();
    let size = id.len();

    for idx in 0..size {
        let mut cur_map = id_map.clone();
        cur_map[idx] = '0';
        result.push(cur_map.into_iter().collect());
    }

    return result;
}

fn main() {
    part_one();
    part_two();
}
