// --- Day 5: Alchemical Reduction ---
// You've managed to sneak in to the prototype suit manufacturing lab. The Elves
// are making decent progress, but are still struggling with the suit's size
// reduction capabilities.
//
// While the very latest in 1518 alchemical technology might have solved their
// problem eventually, you can do better. You scan the chemical composition of
// the suit's material and discover that it is formed by extremely long polymers
// (one of which is available as your puzzle input).
//
// The polymer is formed by smaller units which, when triggered, react with each
// other such that two adjacent units of the same type and opposite polarity are
// destroyed. Units' types are represented by letters; units' polarity is
// represented by capitalization. For instance, r and R are units with the same
// type but opposite polarity, whereas r and s are entirely different types and
// do not react.
//
// For example:
//
// In aA, a and A react, leaving nothing behind.
// In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
// In abAB, no two adjacent units are of the same type, and so nothing happens.
// In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
// Now, consider a larger example, dabAcCaCBAcCcaDA:
//
// dabAcCaCBAcCcaDA  The first 'cC' is removed.
// dabAaCBAcCcaDA    This creates 'Aa', which is removed.
// dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
// dabCBAcaDA        No further actions can be taken.
//
// After all possible reactions, the resulting polymer contains 10 units.
//
// How many units remain after fully reacting the polymer you scanned? (Note: in
// this puzzle and others, the input is large; if you copy/paste your input,
// make sure you get the whole thing.)

extern crate utils;

use std::collections::HashMap;

fn should_be_destroyed(prev: char, next: char) -> bool {
    let are_polar =
        prev.is_uppercase() && next.is_lowercase() || prev.is_lowercase() && next.is_uppercase();

    if are_polar {
        prev.to_ascii_uppercase() == next.to_ascii_uppercase()
    } else {
        false
    }
}

fn cleanup_polymer(polymer: &String) -> (String, HashMap<char, u32>) {
    let mut result = Vec::new();
    let mut counters = HashMap::new();

    for unit in polymer.chars() {
        let mut collapse = false;

        if let Some(last_unit) = result.last() {
            collapse = should_be_destroyed(*last_unit, unit);
        }

        if collapse {
            result.pop();
            counters
                .entry(unit.to_ascii_uppercase())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        } else {
            result.push(unit);
        }
    }

    let final_polymer: String = result.iter().collect();

    (final_polymer, counters)
}

fn part_one(polymer: &String) {
    let (final_polymer, _) = cleanup_polymer(&polymer);

    println!("--- Part 1 ---");
    // println!("Final polymer: {:?}", final_polymer);
    println!("Final length: {}", final_polymer.len());
}

// --- Part Two ---
// Time to improve the polymer.

// One of the unit types is causing problems; it's preventing the polymer from
// collapsing as much as it should. Your goal is to figure out which unit type
// is causing the most problems, remove all instances of it (regardless of
// polarity), fully react the remaining polymer, and measure its length.

// For example, again using the polymer dabAcCaCBAcCcaDA from above:

// Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
// Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
// Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
// Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.

// In this example, removing all C/c units was best, producing the answer 4.

// What is the length of the shortest polymer you can produce by removing all
// units of exactly one type and fully reacting the result?

fn part_two(polymer: &String) {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut min_length = polymer.len();
    let mut bad_unit = None;

    for letter_to_remove in alphabet {
        let candidate: String = polymer
            .chars()
            .filter(|el| el.to_ascii_lowercase() != letter_to_remove)
            .collect();

        let (candidate_reacted, _) = cleanup_polymer(&candidate);

        if candidate_reacted.len() < min_length {
            min_length = candidate_reacted.len();
            bad_unit = Some(letter_to_remove);
        }
    }

    println!("--- Part 2 ---");
    println!("Min len: {:?}", min_length);
    println!("Bad Unit: {:?}", bad_unit);
}

fn main() {
    let mut polymer = utils::read_puzzle_input(5);
    polymer.pop(); // removing final newline

    part_one(&polymer);
    part_two(&polymer);
}
