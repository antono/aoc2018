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
        Point { id: String::from(id), x, y }
    }

    pub fn rectilinear_distance(&self, b: &Point) -> u32 {
        (
            (self.x as i32 - b.x as i32).abs() +
            (self.y as i32 - b.y as i32).abs()
        ) as u32
    }

    //
    // Return closest point or multiple points if ditance to all of them is the same...
    //
    pub fn find_closest_points<'a>(&self, points: &'a Vec<Point>) -> Vec<&'a Point> {
        let mut closest_points = vec![]; // HashSet maybe?
        let mut closest_distance = 1000;

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
}

impl World {
    fn new(points: Vec<Point>) -> World {
        let map = vec![vec![String::from("."); 10]; 10]; // 10x10
        let proximity_map = vec![vec![String::from("."); 10]; 10]; // 10x10
        let mut world = World { points, map, proximity_map };

        world.build_map();
        world.build_proximity_map();

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

    // returns number and point for biggest island
    fn find_biggest_island(&self) -> (u32, String) {
        let mut not_islands = HashSet::new();
        let mut counts = HashMap::new();

        not_islands.insert(String::from("."));

        for (i, row) in self.proximity_map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                // border points belongs to unlimited islands
                if i == 0 || i == 9 || j == 0 || j == 9 {
                    not_islands.insert(col.to_ascii_lowercase());
                    continue;
                }

                counts
                    .entry(col.to_ascii_lowercase())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }

        let keys: HashSet<String> = counts.keys().cloned().collect();
        let island_keys = keys.difference(&not_islands);

        let mut max_island = String::from(".");
        let mut max_island_size = 0;

        for key in island_keys {
            if counts[key] > max_island_size {
                max_island = key.clone();
                max_island_size = counts[key];
            }
        }

        println!("{:#?}", not_islands);
        println!("{:#?}", counts);
        println!("{:#?}", keys.difference(&not_islands));

        (max_island_size, max_island)
    }
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
        let points = vec![
            Point::new("A", 0, 0),
        ];

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

        let world = World::new(points);

        let mut proximity_map_str = String::new();
        for row in world.proximity_map {
            for col in row {
                proximity_map_str.push_str(&col);
            }
            proximity_map_str.push_str("\n");
        }

        let expected_proximity_map_str = indoc!["
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
        "];

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

        let world = World::new(points);

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

        let expected_map_str = indoc!["
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
        "];

        let expected_proximity_map_str = indoc!["
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
        "];

        assert_eq!(expected_map_str, map_str);
        assert_eq!(expected_proximity_map_str, proximity_map_str);

        // println!("{}", proximity_map_str);
        // println!("{}", expected_proximity_map_str);
    }
}

fn main() {
    // let a = Point { id: "A", x: 1, y: 7 };
    // let b = Point { id: "B", x: 8, y: 4 };

    // rectilinear_distance(&a, &b);
}
