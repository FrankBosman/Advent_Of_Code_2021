mod priority;

use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};
use Advent_Of_Code_2021::{read_in, Point};
use colored::Colorize;
use crate::priority::Priority;

const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 15".bright_green().bold());
    // x + y * size.0
    let (grid, size) = read_in::parse_grid(if TEST_MODE { "src/day15/testInput.txt" } else { "src/day15/input.txt" });
    let grid: Vec<Node> = grid.iter().enumerate().map(|(i, risk)| { Node::new(i, risk, &size) }).collect();

    part1(&grid, &size);
    part2(&grid, &size);
}

fn part1(grid: &Vec<Node>, size: &(usize, usize)) {
    println!("{}", "Part 1".green());
    let answer = dijkstra(0, grid.len() - 1, grid, size);

    let answer = answer.unwrap_or(usize::MAX);
    let answer = if answer == usize::MAX { "x".red() } else { answer.to_string().yellow() };
    println!("- {}", answer);
}

fn part2(grid: &Vec<Node>, size: &(usize, usize)) {
    let multiplier = 5usize;
    let mut new_grid: Vec<Node> = Vec::with_capacity(grid.len() * multiplier * multiplier);

    for y in 0..(size.1 * multiplier) {
        for x in 0..(size.0 * multiplier) {
            let actual_x = x % size.0;
            let actual_y = y % size.1;
            let repeat = x / size.0 + y / size.1;
            let mut risk = (&grid[actual_x + actual_y * size.0]).risk + repeat;
            if risk > 9 {
                risk -= 9;
            }

            let node = Node {pos: Point::new(x as i16, y as i16), risk };
            new_grid.push(node);
        }
    }

    let new_size = (size.0 * multiplier, size.1 * multiplier);

    let answer = dijkstra(0, new_grid.len() - 1, &new_grid, &new_size);

    let answer = answer.unwrap_or(usize::MAX);
    let answer = if answer == usize::MAX { "x".red() } else { answer.to_string().yellow() };

    println!("{}", "Part 2".green());
    println!("- {}", answer.to_string().yellow());
}

fn dijkstra(start: usize, target: usize, grid: &Vec<Node>, size: &(usize, usize)) -> Option<usize> {
    let mut costs: Vec<usize> = (0..grid.len()).map(|_| usize::MAX).collect();
    // let heuristic: Vec<usize> = (0..grid.len()).map(|i| size.0 - (i % size.0) + size.1 - (i / size.0) - 2).collect();

    // A* path finding algorithm
    let mut frontier = BinaryHeap::new();

    // We're at `start`, with a zero cost
    costs[start] = 0;
    frontier.push(Priority::new(0, start));

    while let Some(Priority { cost, value: index }) = frontier.pop() {
        if index == target { return Some(cost); }  // Found target
        if cost > costs[index] { continue; }  // Already found a faster path to this node
        let node = &grid[index];
        for neighbour in node.neighbours(size) {
            let new_cost = (&grid[neighbour]).risk + cost;
            if new_cost < costs[neighbour] {
                frontier.push(Priority::new(new_cost, neighbour));
                costs[neighbour] = new_cost;
            }
        }
    }

    None
}

struct Node {
    pos: Point,
    risk: usize,
}

impl Node {
    fn new(index: usize, risk: &usize, size: &(usize, usize)) -> Self {
        Node { pos: Point::from_index(index, size), risk: *risk }
    }

    fn neighbours(&self, size: &(usize, usize)) -> Vec<usize> {
        self.pos.get_neighbours(size, false).iter().map(|point| point.index(size)).collect()
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.risk.to_string())
    }
}