use colored::Colorize;
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use Advent_Of_Code_2021::read_in;
const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 12".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day12/testInput.txt" } else { "src/day12/input.txt" });

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let cave = construct_cave(lines);
    let visited = HashSet::from(["start".into()]);
    let answer = cave.get("start").unwrap().borrow().transverse(&cave, visited);

    println!("{}","Part 1".green());
    println!("- {}", answer.to_string().yellow());
}

fn part2(lines: &Vec<String>) {
    let cave = construct_cave(lines);
    let visited = HashSet::from(["start".into()]);
    let answer = cave.get("start").unwrap().borrow().transverse2(&cave, visited, false);

    println!("{}","Part 2".green());
    println!("- {}", answer.to_string().yellow());
}

fn construct_cave(lines: &Vec<String>) -> HashMap<String, RefCell<Node>> {
    let mut cave: HashMap<String, RefCell<Node>> = HashMap::new();
    for line in lines { // start-b
        let (name1, name2) =  line.split_once("-").unwrap();

        // create obj if they do not exist yet
        if !cave.contains_key(name1.into()) {cave.insert(name1.into(), RefCell::new(Node::new(name1.into())));}
        if !cave.contains_key(name2.into()) {cave.insert(name2.into(), RefCell::new(Node::new(name2.into())));}

        // Get the nodes
        let node1 = cave.get(name1.into()).unwrap().borrow_mut();
        let node2 = cave.get(name2.into()).unwrap().borrow_mut();

        // Connect them
        Node::connect(node1, node2);
    }

    cave
}

struct Node {
    name: String,
    connections: Vec<String>,
    is_large: bool,
}

impl Node {
    fn new(name: String) -> Self {
        let is_large = name.chars().last().unwrap().is_ascii_uppercase();
        Self {name, connections: Vec::new(), is_large}
    }

    fn connect(mut node1: RefMut<Node>, mut node2: RefMut<Node>) {
        node1.connections.push(node2.name.clone());
        node2.connections.push(node1.name.clone());
    }

    fn transverse(&self, cave: &HashMap<String, RefCell<Node>>, visited: HashSet<String>) -> u32 {
        if self.name.eq("end") {return 1;}

        let mut sum = 0u32;
        for connection in &self.connections {
            let node = cave.get(connection.into()).unwrap().borrow();

            // If we already visited the small cave, then skip it
            if !node.is_large && visited.contains(connection) { continue; }

            let mut visited_new = visited.clone();
            visited_new.insert(connection.clone());
            sum += node.transverse(cave, visited_new);
        }

        sum
    }

    fn transverse2(&self, cave: &HashMap<String, RefCell<Node>>, visited: HashSet<String>, visited_small: bool) -> u32 {
        if self.name.eq("end") {return 1;}

        let mut sum = 0u32;
        for connection in &self.connections {
            if connection.eq("start") {continue;}
            let node = cave.get(connection.into()).unwrap().borrow();

            // If we already visited the small cave, then skip it
            if !node.is_large && visited.contains(connection) && visited_small { continue; }

            let mut visited_new = visited.clone();

            let visit_small_twice = !node.is_large && visited.contains(connection) || visited_small;

            visited_new.insert(connection.clone());
            sum += node.transverse2(cave, visited_new, visit_small_twice);
        }

        sum
    }
}