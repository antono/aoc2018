// --- Day 3: No Matter How You Slice It ---
// The Elves managed to locate the chimney-squeeze prototype fabric for Santa's
// suit (thanks to someone who helpfully wrote its box IDs on the wall of the
// warehouse in the middle of the night). Unfortunately, anomalies are still
// affecting them - nobody can even agree on how to cut the fabric.
//
// The whole piece of fabric they're working on is a very large square - at
// least 1000 inches on each side.
//
// Each Elf has made a claim about which area of fabric would be ideal for
// Santa's suit. All claims have an ID and consist of a single rectangle with
// edges parallel to the edges of the fabric. Each claim's rectangle is defined
// as follows:
//
// - The number of inches between the left edge of the fabric and the left edge of the rectangle.
// - The number of inches between the top edge of the fabric and the top edge of the rectangle.
// - The width of the rectangle in inches.
// - The height of the rectangle in inches.
//
// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3
// inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4
// inches tall. Visually, it claims the square inches of fabric represented by #
// (and ignores the square inches of fabric represented by .) in the diagram
// below:
//
// ...........
// ...........
// ...#####...
// ...#####...
// ...#####...
// ...#####...
// ...........
// ...........
// ...........
//
// The problem is that many of the claims overlap, causing two or more claims to
// cover part of the same areas. For example, consider the following claims:
//
// #1 @ 1,3: 4x4
// #2 @ 3,1: 4x4
// #3 @ 5,5: 2x2
// Visually, these claim the following areas:
//
// ........
// ...2222.
// ...2222.
// .11XX22.
// .11XX22.
// .111133.
// .111133.
// ........
//
// The four square inches marked with X are claimed by both 1 and 2. (Claim 3,
// while adjacent to the others, does not overlap either of them.)
//
// If the Elves all proceed with their own plans, none of them will have enough
// fabric. How many square inches of fabric are within two or more claims?

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate utils;

use regex::Regex;

#[derive(Debug)]
struct Claim {
    id: u32,
    offset_x: u32,
    offset_y: u32,
    width: u32,
    height: u32,
}

fn parse_claim(input: &str) -> Option<Claim> {
    lazy_static! {
        static ref CLAIM_REGEX: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    match CLAIM_REGEX.captures(input) {
        Some(captures) => Some(Claim {
            id: captures[1].parse().unwrap(),
            offset_x: captures[2].parse().unwrap(),
            offset_y: captures[3].parse().unwrap(),
            width: captures[4].parse().unwrap(),
            height: captures[5].parse().unwrap(),
        }),
        None => None,
    }
}

fn init_map(size: u32) -> Vec<Vec<Vec<u32>>> {
    let mut map = Vec::with_capacity(size as usize);

    for x in 0..size {
        let y_axis = Vec::with_capacity(size as usize);
        map.push(y_axis);
        for _y in 0..size {
            map[x as usize].push(Vec::new());
        }
    }

    return map;
}

fn map_claim(map: &mut Vec<Vec<Vec<u32>>>, claim: &Claim) {
    let min_x = claim.offset_x as usize;
    let max_x = (claim.offset_x + claim.width) as usize;
    let min_y = claim.offset_y as usize;
    let max_y = (claim.offset_y + claim.height) as usize;

    for x in min_x..max_x {
        for y in min_y..max_y {
            map[x][y].push(claim.id);
        }
    }
}

fn map_claims(string: String) -> Vec<Vec<Vec<u32>>> {
    let mut map = init_map(1000);

    for line in string.lines() {
        let claim = parse_claim(line).unwrap();
        map_claim(&mut map, &claim);
    }

    return map;
}

fn part_one() {
    let input = utils::read_puzzle_input(3);
    let map = map_claims(input);

    let mut counter = 0;

    for x in map {
        for intersection in x {
            if intersection.len() >= 2 {
                counter += 1;
            }
        }
    }
    println!(
        "Total inches of fabric within 2 or more claims: {}",
        counter
    );
}

// --- Part Two ---
// Amidst the chaos, you notice that exactly one claim doesn't overlap
// by even a single square inch of fabric with any other claim. If you
// can somehow draw attention to it, maybe the Elves will be able to
// make Santa's suit after all!
//
// For example, in the claims above, only claim 3 is intact after all
// claims are made.
//
// What is the ID of the only claim that doesn't overlap?

use std::collections::HashSet;

fn part_two() {
    let input = utils::read_puzzle_input(3);
    let map = map_claims(input);
    let mut conflicts = HashSet::new();
    let mut singles = HashSet::new();

    for x in map {
        for intersection in x {
            if intersection.len() == 1 {
                singles.insert(intersection.first().unwrap().clone());
            } else if intersection.len() >= 2 {
                for conflicting in intersection {
                    conflicts.insert(conflicting.clone());
                }
            }
        }
    }

    // println!("Conflicts: {:?}", conflicts);
    // println!("Singles: {:?}", singles);

    for x in singles.difference(&conflicts) {
        println!("Non overlaping claim: {}", x);
    }
}

fn main() {
    part_one();
    part_two();
}
