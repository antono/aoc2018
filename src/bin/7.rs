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
//  /    \      \     / Y \
// C      -->D----->E     Y
//  \           /    \ X /
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
//

// --- Part Two ---
//
// As you're about to begin construction, four of the Elves offer to help. "The sun
// will set soon; it'll go faster if we work together." Now, you need to account
// for multiple people working on steps simultaneously. If multiple steps are
// available, workers should still begin them in alphabetical order.
//
// Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2,
// C=3, and so on. So, step A takes 60+1=61 seconds, while step Z takes 60+26=86
// seconds. No time is required between steps.
//
// To simplify things for the example, however, suppose you only have help from one
// Elf (a total of two workers) and that each step takes 60 fewer seconds (so that
// step A takes 1 second and step Z takes 26 seconds). Then, using the same
// instructions as above, this is how each second would be spent:
//
// Second   Worker 1   Worker 2   Done
//    0        C          .
//    1        C          .
//    2        C          .
//    3        A          F       C
//    4        B          F       CA
//    5        B          F       CA
//    6        D          F       CAB
//    7        D          F       CAB
//    8        D          F       CAB
//    9        D          .       CABF
//   10        E          .       CABFD
//   11        E          .       CABFD
//   12        E          .       CABFD
//   13        E          .       CABFD
//   14        E          .       CABFD
//   15        .          .       CABFDE
//
// Each row represents one second of time. The Second column identifies how many
// seconds have passed as of the beginning of that second. Each worker column shows
// the step that worker is currently doing (or . if they are idle). The Done column
// shows completed steps.
//
// Note that the order of the steps has changed; this is because steps now take
// time to finish and multiple workers can begin multiple steps simultaneously.
//
// In this example, it would take 15 seconds for two workers to complete these
// steps.
//
// With 5 workers and the 60+ second step durations described above, how long will
// it take to complete all of the steps?

extern crate regex;
extern crate utils;

use std::collections::HashMap;
use std::fmt;

use regex::Regex;

#[derive(Clone, Debug)]
struct Edge {
    from: Letter,
    to: Letter,
}

#[derive(Debug, Clone)]
struct DAG {
    edges: Vec<Edge>,
    nodes: HashMap<Letter, Vec<Letter>>,
}

impl DAG {
    fn from_string(input: String) -> DAG {
        let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
        let mut dag: DAG = DAG::new();

        for cap in re.captures_iter(&input) {
            dag.add_edge(Edge {
                from: Letter::from_string(String::from(&cap[1])),
                to: Letter::from_string(String::from(&cap[2])),
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
        self.nodes.entry(edge.from).or_insert(vec![]);
        self.nodes.entry(edge.to).or_default().push(edge.from);
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
    pub fn topological_sort(&mut self) -> Vec<Letter> {
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

    pub fn aoc_sort(&mut self) -> Vec<Letter> {
        let mut output = vec![];
        while let Some(root) = self.next_root() {
            self.complete(root);
            println!("Pushing next node {root}");
            output.push(root);
        }
        output
    }

    pub fn complete(&mut self, node: Letter) {
        let has_key = self.nodes.contains_key(&node);
        let removed = self.nodes.remove(&node);

        println!("{}", self);
        println!("Removed value: {}", has_key);
        println!("Removed {}", node);
        println!("{}", self);
        println!(
            "Keys: {}",
            self.nodes.keys().map(|l| l.char).collect::<String>()
        );
        for (_, incoming) in self.nodes.iter_mut() {
            if let Some(idx) = incoming.iter().position(|i| *i == node) {
                incoming.remove(idx);
                println!("Removing node from incoming: {}", idx)
            }
        }
        if let Some(next_root) = self.next_root() {
            println!("Next: {}", next_root);
        }
    }

    // Root nodes have no incoming edges
    pub fn find_root_nodes(&self) -> Option<Vec<Letter>> {
        let mut roots = vec![];
        for (node, incoming) in self.nodes.iter() {
            if incoming.is_empty() {
                roots.push(node.clone());
            }
        }

        if roots.is_empty() {
            None
        } else {
            roots.sort_by(|a, b| a.char.cmp(&b.char));
            Some(roots)
        }
    }

    pub fn next_root(&self) -> Option<Letter> {
        if let Some(roots) = self.find_root_nodes() {
            Some(roots[0])
        } else {
            None
        }
    }
}

impl fmt::Display for DAG {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DAG: [\n")?;
        for (node, incoming) in self.nodes.iter() {
            let incoming = incoming
                .iter()
                .map(|l| l.as_string())
                .collect::<Vec<String>>()
                .join(", ");
            write!(f, "  {} -> [{}]\n", node, incoming)?;
        }
        write!(f, "]\n")
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Letter {
    char: char,
    seconds: usize,
    in_progress: usize,
}

impl PartialEq for Letter {
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char
    }
}

impl Eq for Letter {}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Letter(char: {}, in_progress: {}, seconds: {}, done: {})",
            self.char,
            self.in_progress,
            self.seconds,
            self.is_done()
        )
    }
}

impl Letter {
    fn from_string(str: String) -> Letter {
        let char = str.chars().next().unwrap().to_ascii_lowercase();
        Self {
            char,
            seconds: 60 + Self::alphabetical_index(&str).unwrap(),
            in_progress: 0,
        }
    }

    fn from_char(char: char) -> Letter {
        let char = char.to_ascii_lowercase();
        Self {
            char,
            seconds: 60 + (char as usize - 'a' as usize) + 1,
            in_progress: 0,
        }
    }

    fn as_string(&self) -> String {
        String::from(self.char)
    }

    fn alphabetical_index(letter: &str) -> Option<usize> {
        if letter.len() == 1 {
            let c = letter.chars().next().unwrap().to_ascii_lowercase();

            if c.is_alphabetic() {
                return Some((c as usize - 'a' as usize) + 1);
            }
        }
        None
    }

    pub fn is_done(self) -> bool {
        self.seconds == self.in_progress
    }
}

#[derive(Debug, Clone)]
struct AssemblyLine {
    workers: Vec<Option<Letter>>,
    dag: DAG,
    seconds: usize,
    completed: Vec<Letter>,
}

impl AssemblyLine {
    fn from_dag(dag: DAG, workers_count: usize) -> AssemblyLine {
        Self {
            workers: vec![None; workers_count],
            dag,
            seconds: 0,
            completed: Vec::new(),
        }
    }

    fn letters_to_string(letters: Vec<Letter>) -> String {
        letters
            .iter()
            .map(|l| l.char.to_ascii_uppercase())
            .collect()
    }

    pub fn assign(&mut self, letter: Option<Letter>, idx: usize) {
        self.workers[idx] = letter;
        if let Some(letter) = self.workers[idx] {
            println!("assigned: {idx}, letter: {letter}");
        }
    }

    pub fn unassign(&mut self, idx: usize) {
        if let Some(letter) = self.workers[idx] {
            println!("unassigned: {idx}, letter: {letter}");
        }
        self.workers[idx] = None
    }

    pub fn next_step(&self) -> Option<Letter> {
        if let Some(roots) = self.dag.find_root_nodes() {
            let in_progress: Vec<Letter> = self
                .workers
                .iter()
                .filter(|w| w.is_some())
                .map(|l| l.unwrap())
                .collect();
            let valid_steps: Vec<Letter> = roots
                .iter()
                .filter(|letter| !in_progress.contains(*letter))
                .map(|l| *l)
                .collect();
            println!(
                "Next valid steps: {}",
                Self::letters_to_string(valid_steps.clone())
            );
            if let Some(letter) = valid_steps.get(0) {
                return Some(*letter);
            } else {
                return None;
            }
        }
        None
    }

    fn complete(&mut self, letter: Letter) {
        self.completed.push(letter);
        self.dag.complete(letter);
    }

    pub fn tick(&mut self) -> Vec<(usize, Letter)> {
        let mut completed: Vec<(usize, Letter)> = vec![];
        for (idx, letter) in self.workers.iter_mut().enumerate() {
            match letter {
                Some(step) => {
                    step.in_progress += 1;
                    println!(
                        "sec: {}, worker: {}, step: {}",
                        self.seconds,
                        idx,
                        step.clone()
                    );
                    if step.is_done() {
                        completed.push((idx, *step));
                    }
                }
                None => {
                    println!("sec: {}, worker: {}, step: None", self.seconds, idx);
                }
            }
        }

        self.seconds += 1;
        completed
    }

    pub fn free_workers(&self) -> Vec<usize> {
        let mut free = vec![];
        for (idx, worker) in self.workers.iter().enumerate() {
            if worker.is_none() {
                free.push(idx);
            }
        }
        free
    }

    pub fn process(&mut self) {
        while self.seconds < 1000 {
            // assign work to free workers
            for idx in self.free_workers() {
                let step = self.next_step();
                self.assign(step, idx);
            }
            // time forward
            let completed = self.tick();
            // unassign completed from workers
            for (idx, letter) in completed {
                self.complete(letter);
                println!(
                    "Completed steps: {}",
                    Self::letters_to_string(self.completed.clone())
                );
                self.unassign(idx);
            }
        }

        print!("Done in {}", self.seconds);
    }
}

fn main() {
    let data = utils::read_puzzle_input(7);
    let mut dag = DAG::from_string(data.clone());

    println!(
        "DAG Sorted: {}",
        AssemblyLine::letters_to_string(dag.aoc_sort())
    );
    // for letter in dag.aoc_sort() {
    //     for edge in dag2.edges.iter() {
    //         if edge.from == letter {
    //             println!("{}, {}", edge.from, edge.to)
    //         }
    //     }
    // }

    // println!("{}", dag2);

    let dag = DAG::from_string(data);
    let mut assembly_line = AssemblyLine::from_dag(dag, 5);
    // dbg!(assembly_line.clone());
    assembly_line.process();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dag_input() -> DAG {
        let data = utils::read_puzzle_input(7);
        DAG::from_string(data)
    }

    fn str_to_letters(string: &str) -> Vec<Letter> {
        string.chars().map(|c| Letter::from_char(c)).collect()
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
        assert_eq!(str_to_letters("C"), dag.find_root_nodes().unwrap());
    }

    #[test]
    fn test_parse_input() {
        let dag = DAG::from_string(String::from(
            "Step R must be finished before step Y can begin.
            Step X must be finished before step Y can begin.",
        ));

        let res0 = dag.edges[0].clone();
        let res1 = dag.edges[1].clone();

        assert_eq!(res0.from, Letter::from_char('R'));
        assert_eq!(res0.to, Letter::from_char('Y'));
        assert_eq!(res1.from, Letter::from_char('X'));
        assert_eq!(res1.to, Letter::from_char('Y'));
    }

    #[test]
    fn test_aoc_sort() {
        // example input
        let mut dag = dag_fixture();
        let res = dag.aoc_sort();
        assert_eq!(res, str_to_letters("CABDFE"));

        // my input
        let mut dag = dag_input();
        let res = dag.aoc_sort();
        assert_eq!(res, str_to_letters("CFMNLOAHRKPTWBJSYZVGUQXIDE"));
    }

    #[test]
    fn test_topological_sort() {
        let mut dag = dag_fixture();
        let sorted = dag.topological_sort();
        assert_eq!(sorted, str_to_letters("CAFBDE"));
    }

    #[test]
    fn test_letter() {
        let letter = Letter::from_char('A');
        assert_eq!(letter.char, 'a');
        assert_eq!(letter.seconds, 61);
        let letter = Letter::from_char('Z');
        assert_eq!(letter.char, 'z');
        assert_eq!(letter.seconds, 86);

        assert_eq!(
            Letter {
                char: 'c',
                seconds: 1,
                in_progress: 2
            },
            Letter {
                char: 'c',
                seconds: 3,
                in_progress: 4,
            }
        );
    }

    // Second   Worker 1   Worker 2   Done
    //    0        C          .
    //    1        C          .
    //    2        C          .
    //    3        A          F       C
    //    4        B          F       CA
    //    5        B          F       CA
    //    6        D          F       CAB
    //    7        D          F       CAB
    //    8        D          F       CAB
    //    9        D          .       CABF
    //   10        E          .       CABFD
    //   11        E          .       CABFD
    //   12        E          .       CABFD
    //   13        E          .       CABFD
    //   14        E          .       CABFD
    //   15        .          .       CABFDE
    //
    #[test]
    fn test_pipeline() {
        let dag = dag_fixture();
        let mut _assembly_line = AssemblyLine::from_dag(dag, 2);
        // assert_eq!(assembly_line.workers[0].unwrap().char, 'c')
    }
}
