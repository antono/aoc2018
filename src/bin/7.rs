// --- Day 7: The Sum of Its Parts ---
//
// You find yourself standing on a snow-covered coastline; apparently, you
// landed a little off course. The region is too hilly to see the North Pole
// from here, but you do spot some Elves that seem to be trying to unpack
// something that washed ashore. It's quite cold out, so you decide to risk
// creating a paradox by asking them for directions.
//
// "Oh, are you the search party?" Somehow, you can understand whatever Elves
// from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the
// device on your wrist also be a translator? "Those clothes don't look very
// warm; take this." They hand you a heavy coat.
//
// "We do need to find our way back to the North Pole, but we have higher
// priorities at the moment. You see, believe it or not, this box contains
// something that will solve all of Santa's transportation problems - at least,
// that's what it looks like from the pictures in the instructions." It doesn't
// seem like they can read whatever language it's in, but you can: "Sleigh kit.
// Some assembly required."
//
// "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at
// once!" They start excitedly pulling more parts out of the box.
//
// The instructions specify a series of steps and requirements about which steps
// must be finished before others can begin (your puzzle input). Each step is
// designated by a single letter. For example, suppose you have the following
// instructions:
//
// Step C must be finished before step A can begin.
// Step C must be finished before step F can begin.
// Step A must be finished before step B can begin.
// Step A must be finished before step D can begin.
// Step B must be finished before step E can begin.
// Step D must be finished before step E can begin.
// Step F must be finished before step E can begin.
//
// Visually, these requirements look like this:
//
//   -->A--->B--
//  /    \      \
// C      -->D----->E
//  \           /
//   ---->F-----
//
// Your first goal is to determine the order in which the steps should be
// completed. If more than one step is ready, choose the step which is first
// alphabetically. In this example, the steps would be completed as follows:
//
// - Only C is available, and so it is done first.
// - Next, both A and F are available. A is first alphabetically, so it is done
// - next.
// - Then, even though F was available earlier, steps B and D are now also
//   available, and B is the first alphabetically of the three.
// - After that, only D and F are available. E is not available because only some
//   of its prerequisites are complete. Therefore, D is completed next.
// - F is the only choice, so it is done next.
// - Finally, E is completed.
//
// So, in this example, the correct order is CABDFE.
//
// In what order should the steps in your instructions be completed?

extern crate regex;
extern crate utils;

use std::collections::HashMap;
use std::fmt;

use regex::Regex;

#[derive(Clone, Debug)]
struct Edge {
    from: String,
    to: String,
}

struct DAG {
    edges: Vec<Edge>,
    nodes: HashMap<String, Vec<String>>,
}

impl DAG {
    fn from_string(input: String) -> DAG {
        let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
        let mut dag: DAG = DAG::new();

        for cap in re.captures_iter(&input) {
            dag.add_edge(Edge {
                from: String::from(&cap[1]),
                to: String::from(&cap[2]),
            });
        }

        dag
    }

    pub fn new() -> Self {
        Self {
            edges: vec![],
            nodes: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge.clone());
        self.nodes.entry(edge.from.clone()).or_insert(vec![]);
        self.nodes
            .entry(edge.to)
            .or_default()
            .push(edge.from.clone());
    }

    // L ← Empty list that will contain the sorted elements
    // S ← Set of all nodes with no incoming edge
    //
    // while S is not empty do
    //     remove a node n from S
    //     add n to L
    //     for each node m with an edge e from n to m do
    //         remove edge e from the graph
    //         if m has no other incoming edges then
    //             insert m into S
    //
    pub fn topological_sort(&mut self) -> Vec<String> {
        let mut output = Vec::new();
        while let Some(root_nodes) = self.find_root_nodes() {
            println!("{}", self);
            for node in root_nodes.iter() {
                self.nodes.remove(node);
                output.push(node.clone());
                println!("Pushing node {node}");
                for (_, incoming) in self.nodes.iter_mut() {
                    if let Some(idx) = incoming.iter().position(|i| i == node) {
                        incoming.remove(idx);
                    }
                }
            }
        }
        output
    }

    pub fn aoc_sort(&mut self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        while let Some(root) = self.next_root() {
            self.nodes.remove(&root);
            println!("Pushing next node {root}");
            for (_, incoming) in self.nodes.iter_mut() {
                if let Some(idx) = incoming.iter().position(|i| i == &root) {
                    incoming.remove(idx);
                }
            }
            output.push(root.clone());
        }
        output
    }

    // Root nodes have no incoming edges
    pub fn find_root_nodes(&self) -> Option<Vec<String>> {
        let mut roots = vec![];
        for (node, incoming) in self.nodes.iter() {
            if incoming.is_empty() {
                roots.push(node.clone());
            }
        }

        if roots.is_empty() {
            None
        } else {
            roots.sort_by(|a, b| a.cmp(b));
            Some(roots)
        }
    }

    pub fn next_root(&self) -> Option<String> {
        if let Some(roots) = self.find_root_nodes() {
            Some(roots[0].clone())
        } else {
            None
        }
    }
}

impl fmt::Display for DAG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DAG: [\n")?;
        for (node, incoming) in self.nodes.iter() {
            write!(f, "  {} -> [{}]\n", node, incoming.join(", "))?;
        }
        write!(f, "]\n");
        write!(
            f,
            "Roots: {}",
            self.find_root_nodes()
                .expect("roots expected")
                .clone()
                .join(", ")
        )
    }
}

fn main() {
    let data = utils::read_puzzle_input(7);
    let mut dag = DAG::from_string(data);

    println!("DAG Sorted: {}", dag.aoc_sort().join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dag_input() -> DAG {
        let data = utils::read_puzzle_input(7);
        DAG::from_string(data)
    }

    fn dag_fixture() -> DAG {
        // Visually, these requirements look like this:
        //
        //   -->A--->B--
        //  /    \      \
        // C      -->D----->E
        //  \           /
        //   ---->F-----
        //
        DAG::from_string(String::from(
            "
            Step C must be finished before step A can begin.
            Step C must be finished before step F can begin.
            Step A must be finished before step B can begin.
            Step A must be finished before step D can begin.
            Step B must be finished before step E can begin.
            Step D must be finished before step E can begin.
            Step F must be finished before step E can begin.
            ",
        ))
    }

    #[test]
    fn test_find_root_nodes() {
        let dag = dag_fixture();

        assert_eq!(Some(vec![String::from("C")]), dag.find_root_nodes());
    }

    #[test]
    fn test_parse_input() {
        let dag = DAG::from_string(String::from(
            "Step R must be finished before step Y can begin.
            Step X must be finished before step Y can begin.",
        ));

        let res0 = dag.edges[0].clone();
        let res1 = dag.edges[1].clone();

        assert_eq!(res0.from, String::from('R'));
        assert_eq!(res0.to, String::from('Y'));
        assert_eq!(res1.from, String::from('X'));
        assert_eq!(res1.to, String::from('Y'));
    }

    #[test]
    fn test_aoc_sort() {
        let mut dag = dag_fixture();

        println!("{}", dag);
        assert_eq!(dag.aoc_sort().join(""), "CABDFE")
    }

    #[test]
    fn test_topological_sort() {
        let mut dag = dag_fixture();

        println!("{}", dag);
        assert_eq!(dag.topological_sort().join(""), "CAFBDE")
    }
}
