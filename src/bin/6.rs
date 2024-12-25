// --- Day 6: Chronal Coordinates ---
// The device on your wrist beeps several times, and once again you feel like
// you're falling.
//
// "Situation critical," the device announces. "Destination indeterminate.
// Chronal interference detected. Please specify new target coordinates."
//
// The device then produces a list of coordinates (your puzzle input). Are they
// places it thinks are safe or dangerous? It recommends you check manual page
// 729. The Elves did not give you a manual.
//
// If they're dangerous, maybe you can minimize the danger by finding the
// coordinate that gives the largest distance from the other points.
//
// Using only the Manhattan distance, determine the area around each coordinate
// by counting the number of integer X,Y locations that are closest to that
// coordinate (and aren't tied in distance to any other coordinate).
//
// Your goal is to find the size of the largest area that isn't infinite. For
// example, consider the following list of coordinates:
//
// 1, 1
// 1, 6
// 8, 3
// 3, 4
// 5, 5
// 8, 9
//
// If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:
//
// ..........
// .A........
// ..........
// ........C.
// ...D......
// .....E....
// .B........
// ..........
// ..........
// ........F.
//
// This view is partial - the actual grid extends infinitely in all directions.
// Using the Manhattan distance, each location's closest coordinate can be
// determined, shown here in lowercase:
//
// aaaaa.cccc
// aAaaa.cccc
// aaaddecccc
// aadddeccCc
// ..dDdeeccc
// bb.deEeecc
// bBb.eeee..
// bbb.eeefff
// bbb.eeffff
// bbb.ffffFf
//
// Locations shown as . are equally far from two or more coordinates, and so
// they don't count as being closest to any.
//
// In this example, the areas of coordinates A, B, C, and F are infinite - while
// not shown here, their areas extend forever outside the visible grid. However,
// the areas of coordinates D and E are finite: D is closest to 9 locations, and
// E is closest to 17 (both including the coordinate's location itself).
// Therefore, in this example, the size of the largest area is 17.
//
// What is the size of the largest area that isn't infinite?
//
extern crate indoc;
extern crate utils;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Clone)]
pub struct Point {
    id: String, // use &str ?
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub fn new(id: &str, x: i32, y: i32) -> Point {
        Point {
            id: String::from(id),
            x,
            y,
        }
    }

    pub fn rectilinear_distance(&self, other: &Point) -> u32 {
        ((self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs()) as u32
    }

    pub fn sum_distances(&self, points: Vec<Point>) -> u32 {
        points
            .iter()
            .map(|point| self.rectilinear_distance(&point))
            .sum()
    }

    //
    // Return closest point or multiple points if ditance to all of them is the same...
    //
    pub fn find_closest_points<'a>(&self, points: &'a Vec<Point>) -> Vec<&'a Point> {
        let mut closest_points = vec![]; // HashSet maybe?
        let mut closest_distance = std::u32::MAX;

        for candidate in points {
            let distance = self.rectilinear_distance(candidate);

            if distance < closest_distance {
                closest_distance = distance;
                closest_points.clear(); // better candidate found!
                closest_points.push(candidate);
            }

            if distance == closest_distance && !closest_points.contains(&candidate) {
                closest_points.push(candidate);
            }
        }

        closest_points
    }
}

struct World {
    points: Vec<Point>,
    map: Vec<Vec<String>>,
    proximity_map: Vec<Vec<String>>,
    closest_map: Vec<Vec<String>>,
}

impl World {
    fn new(points: Vec<Point>, width: usize, height: usize) -> World {
        let map = vec![vec![String::from("."); width]; height];
        let proximity_map = vec![vec![String::from("."); width]; height]; // 10x10
        let closest_map = vec![vec![String::from("."); width]; height]; // 10x10

        let mut world = World {
            points,
            map,
            proximity_map,
            closest_map,
        };

        world.build_map();
        world.build_proximity_map();
        world.build_closest_map(10_000); // 10000 comes from requirements

        world
    }

    fn build_map(&mut self) {
        for point in self.points.iter() {
            self.map[point.y as usize][point.x as usize] = point.id.clone();
        }
    }

    fn build_proximity_map(&mut self) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col != "." {
                    self.proximity_map[i][j] = col.clone();
                    continue;
                }

                let point = Point::new("", j as i32, i as i32);
                let closest = point.find_closest_points(&self.points);

                if closest.len() > 1 {
                    self.proximity_map[i][j] = String::from(".");
                } else if closest.len() == 1 {
                    let p = closest[0];
                    self.proximity_map[i][j] = p.id.to_ascii_lowercase();
                }
            }
        }
    }

    fn build_closest_map(&mut self, max_allowed_distance: u32) {
        for (i, row) in self.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let point = Point::new("?", j as i32, i as i32);
                let total_distance = point.sum_distances(self.points.clone());

                if total_distance < max_allowed_distance {
                    self.closest_map[i][j] = String::from("#");
                } else {
                    self.closest_map[i][j] = String::from(cell);
                }
            }
        }
    }

    fn closest_island_size(&self) -> u32 {
        self.closest_map
            .iter()
            .flat_map(|vec| vec.iter())
            .map(|cell| if cell == "#" { 1 } else { 0 })
            .sum()
    }

    // returns number and point for biggest island
    fn find_biggest_island(&self) -> (u32, String) {
        let mut unlimited_islands = HashSet::new();
        let mut counts = HashMap::new();

        unlimited_islands.insert(String::from("."));

        for (i, row) in self.proximity_map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                // border points belongs to unlimited islands
                if i == 0 || i == 9 || j == 0 || j == 9 {
                    unlimited_islands.insert(col.to_ascii_lowercase());
                    continue;
                }

                counts
                    .entry(col.to_ascii_lowercase())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }

        // let max = counts.iter().filter(|(k,v)| !unlimited_islands.contains(*k)).max_by_key(|(k,v)| v).unwrap();

        // println!("{:?}", max)

        let keys: HashSet<String> = counts.keys().cloned().collect();
        let real_islands = keys.difference(&unlimited_islands);

        let mut max_island = String::from(".");
        let mut max_island_size = 0;

        for key in real_islands {
            if counts[key] > max_island_size {
                max_island = key.clone();
                max_island_size = counts[key];
            }
        }

        // println!("{:#?}", unlimited_islands);
        // println!("{:#?}", counts);
        // println!("{:#?}", keys.difference(&unlimited_islands));

        (max_island_size, max_island)
    }
}

fn part_one(points: Vec<Point>, width: usize, height: usize) {
    let world = World::new(points, width + 1, height + 1);
    let (biggest_island_size, _) = world.find_biggest_island();

    println!("--- Part 1 ---");
    println!("Biggest island size: {:?}", biggest_island_size);
}

// --- Part Two ---
//
// On the other hand, if the coordinates are safe, maybe the best you can do is
// try to find a region near as many coordinates as possible.
//
// For example, suppose you want the sum of the Manhattan distance to all of the
// coordinates to be less than 32. For each location, add up the distances to
// all of the given coordinates; if the total of those distances is less than
// 32, that location is within the desired region. Using the same coordinates as
// above, the resulting region looks like this:
//
// ..........
// .A........
// ..........
// ...###..C.
// ..#D###...
// ..###E#...
// .B.###....
// ..........
// ..........
// ........F.
//
// In particular, consider the highlighted location 4,3 located at the top
// middle of the region. Its calculation is as follows, where abs() is the
// absolute value function:
//
//     Distance to coordinate A: abs(4-1) + abs(3-1) =  5
//     Distance to coordinate B: abs(4-1) + abs(3-6) =  6
//     Distance to coordinate C: abs(4-8) + abs(3-3) =  4
//     Distance to coordinate D: abs(4-3) + abs(3-4) =  2
//     Distance to coordinate E: abs(4-5) + abs(3-5) =  3
//     Distance to coordinate F: abs(4-8) + abs(3-9) = 10
//     Total distance: 5 + 6 + 4 + 2 + 3 + 10 = 30
//
// Because the total distance to all coordinates (30) is less than 32, the
// location is within the region.
//
// This region, which also includes coordinates D and E, has a total size of 16.
//
// Your actual region will need to be much larger than this example, though,
// instead including all locations with a total distance of less than 10000.
//
// What is the size of the region containing all locations which have a total
// distance to all given coordinates of less than 10000?

fn part_two(points: Vec<Point>, width: usize, height: usize) {
    let world = World::new(points, width + 1, height + 1);
    let size = world.closest_island_size();

    println!("--- Part 2 ---");
    println!("Closest island size: {:?}", size);
}

fn main() {
    let input = utils::read_puzzle_input(6);
    let mut points = vec![];

    for (i, line) in input.lines().enumerate() {
        let xy: Vec<i32> = line
            .split(",")
            .map(|s| s.trim())
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let point = Point::new(&format!("{}", i), xy[0], xy[1]);
        points.push(point.clone());
    }

    let width = points.iter().max_by_key(|p| p.x).unwrap().x as usize;
    let height = points.iter().max_by_key(|p| p.y).unwrap().y as usize;

    part_one(points.clone(), width, height);
    part_two(points.clone(), width, height);
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_rectilinear_distance() {
        let a = Point::new("A", 1, 1);
        let b = Point::new("B", 1, 6);

        assert_eq!(5, a.rectilinear_distance(&b));
    }

    #[test]
    fn test_find_closest_points_one_candidate() {
        let points = vec![Point::new("A", 0, 0)];

        let point = Point::new("B", 1, 1); // both A and C equally close
        let closest = point.find_closest_points(&points);
        assert_eq!(1, closest.len());
        assert_eq!(true, closest.contains(&&points[0]));
    }

    #[test]
    fn test_find_closest_points_two_candidates() {
        // A..
        // ..C
        let points = vec![
            Point::new("A", 0, 0),
            // Point::new("B", 1, 1),
            Point::new("C", 2, 2),
        ];

        let point = Point::new("B", 1, 1); // both A and C equally close
        let closest = point.find_closest_points(&points);

        assert_eq!(2, closest.len());
        assert_eq!(true, closest.contains(&&points[0]));
        assert_eq!(true, closest.contains(&&points[1]));
    }

    #[test]
    fn test_world_build_proximity_map() {
        let points = vec![
            Point::new("A", 1, 1),
            Point::new("B", 1, 6),
            Point::new("C", 8, 3),
            Point::new("D", 3, 4),
            Point::new("E", 5, 5),
            Point::new("F", 8, 9),
        ];

        let world = World::new(points, 10, 10);

        let mut proximity_map_str = String::new();
        for row in world.proximity_map {
            for col in row {
                proximity_map_str.push_str(&col);
            }
            proximity_map_str.push_str("\n");
        }

        let expected_proximity_map_str = indoc![
            "
                aaaaa.cccc
                aAaaa.cccc
                aaaddecccc
                aadddeccCc
                ..dDdeeccc
                bb.deEeecc
                bBb.eeee..
                bbb.eeefff
                bbb.eeffff
                bbb.ffffFf
            "
        ];

        assert_eq!(proximity_map_str, expected_proximity_map_str);
    }

    #[test]
    fn test_world_find_biggest_island() {
        let points = vec![
            Point::new("A", 1, 1),
            Point::new("B", 1, 6),
            Point::new("C", 8, 3),
            Point::new("D", 3, 4),
            Point::new("E", 5, 5),
            Point::new("F", 8, 9),
        ];

        let world = World::new(points, 10, 10);

        let (biggest_island_size, biggest_island) = world.find_biggest_island();

        // aaaaa.cccc
        // aAaaa.cccc
        // aaaddecccc
        // aadddeccCc
        // ..dDdeeccc
        // bb.deEeecc
        // bBb.eeee..
        // bbb.eeefff
        // bbb.eeffff
        // bbb.ffffFf

        assert_eq!("e".to_string(), biggest_island);
        assert_eq!(17, biggest_island_size);
    }

    #[test]
    // Using data from requirements...
    fn test_map_distances() {
        let mut map = vec![vec![String::from("."); 10]; 10]; // 10x10

        let points = vec![
            Point::new("A", 1, 1),
            Point::new("B", 1, 6),
            Point::new("C", 8, 3),
            Point::new("D", 3, 4),
            Point::new("E", 5, 5),
            Point::new("F", 8, 9),
        ];

        for point in points.iter() {
            map[point.y as usize][point.x as usize] = point.id.clone();
        }

        let mut map_str = String::new();
        for row in &map {
            for col in row {
                map_str.push_str(&col);
            }
            map_str.push_str("\n");
        }

        let mut proximity_map_str = String::new();

        for (i, row) in map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col != "." {
                    proximity_map_str.push_str(col);
                    continue;
                }

                let point = Point::new("", j as i32, i as i32);
                let closest = point.find_closest_points(&points);

                if closest.len() > 1 {
                    proximity_map_str.push_str(".");
                } else if closest.len() == 1 {
                    let p = closest[0];
                    proximity_map_str.push_str(&p.id.to_ascii_lowercase());
                }
            }
            proximity_map_str.push_str("\n");
        }

        let expected_map_str = indoc![
            "
                ..........
                .A........
                ..........
                ........C.
                ...D......
                .....E....
                .B........
                ..........
                ..........
                ........F.
            "
        ];

        let expected_proximity_map_str = indoc![
            "
                aaaaa.cccc
                aAaaa.cccc
                aaaddecccc
                aadddeccCc
                ..dDdeeccc
                bb.deEeecc
                bBb.eeee..
                bbb.eeefff
                bbb.eeffff
                bbb.ffffFf
            "
        ];

        assert_eq!(expected_map_str, map_str);
        assert_eq!(expected_proximity_map_str, proximity_map_str);
    }

    #[test]
    fn test_sum_distances_to_all_points() {
        let test_points = vec![
            Point::new("A", 1, 1),
            Point::new("B", 1, 6),
            Point::new("C", 8, 3),
            Point::new("D", 3, 4),
            Point::new("E", 5, 5),
            Point::new("F", 8, 9),
        ];

        let some_point = Point::new("#", 4, 3);

        let sum = some_point.sum_distances(test_points);

        assert_eq!(sum, 30)
    }

    #[test]
    fn test_build_closest_map() {
        let test_points = vec![
            Point::new("A", 1, 1),
            Point::new("B", 1, 6),
            Point::new("C", 8, 3),
            Point::new("D", 3, 4),
            Point::new("E", 5, 5),
            Point::new("F", 8, 9),
        ];
        let mut world = World::new(test_points, 10, 10);

        world.build_closest_map(32);

        let mut closest_map_str = String::new();

        for row in world.closest_map.iter() {
            for cell in row.iter() {
                closest_map_str.push_str(cell);
            }
            closest_map_str.push_str(&"\n");
        }

        let expected_closest_map_str = indoc![
            "
                ..........
                .A........
                ..........
                ...###..C.
                ..#D###...
                ..###E#...
                .B.###....
                ..........
                ..........
                ........F.
            "
        ];

        assert_eq!(expected_closest_map_str, expected_closest_map_str);

        let closest_island_size = world.closest_island_size();
        assert_eq!(closest_island_size, 16);
    }
}
