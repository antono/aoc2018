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

fn should_be_destroyed(prev: char, next: char) -> bool {
    let are_polar = prev.is_uppercase() && next.is_lowercase() || prev.is_lowercase() && next.is_uppercase();

    if are_polar {
        prev.to_ascii_uppercase() == next.to_ascii_uppercase()
    } else {
        false
    }
}

fn part_one() {
    let mut polymer = utils::read_puzzle_input(5);
    let mut result = Vec::new();

    polymer.pop(); // removing final newline

    for unit in polymer.chars() {
        let mut collapse = false;

        if let Some(last_unit) = result.last() {
            collapse = should_be_destroyed(*last_unit, unit);
        }

        if collapse {
            result.pop();
        } else {
            result.push(unit);
        }
    }


    let final_polymer: String = result.into_iter().collect();

    println!("Final polymer: {:?}", final_polymer);
    println!("Final length: {:?}", final_polymer.len());
}

fn main() {
    part_one();
}
