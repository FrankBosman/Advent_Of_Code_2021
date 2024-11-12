use std::fmt::{Display};
use Advent_Of_Code_2021::read_in;
use colored::Colorize;
use Advent_Of_Code_2021::helpers::print;

const TEST_MODE: bool = false;

#[derive(Clone)]
struct Coord {
    pub id: usize,
    pub val: usize,
    pub lowest_point: bool,
    pub basin: bool,
    pub basin_id: usize,
}
impl Coord {
    pub fn clone(&self) -> Coord {
        Coord { id: self.id, val: self.val, lowest_point: self.lowest_point, basin: self.basin, basin_id: 0 }
    }
    pub fn from(id: usize, character: char) -> Self {
        let val = character.to_digit(10).unwrap() as usize;
        Self { id, val, lowest_point: false, basin: false, basin_id: 0 }
    }

    pub fn update(&self, field: &Vec<Coord>, size: (usize, usize)) -> Self {
        let mut clonned = self.clone();
        clonned.lowest_point = self.is_lowest(field, size);
        clonned
    }
    fn get_surrounding(&self, size: (usize, usize)) -> Vec<usize> {
        let coords = self.get_coords(size);
        let mut surrounding: Vec<usize> = Vec::new();

        if coords.0 > 0 {
            surrounding.push(self.to_index(coords.0 - 1, coords.1, size));
        }
        if coords.1 > 0 {
            surrounding.push(self.to_index(coords.0, coords.1 - 1, size));
        }
        if coords.0 < size.0 - 1 {
            surrounding.push(self.to_index(coords.0 + 1, coords.1, size));
        }
        if coords.1 < size.1 - 1 {
            surrounding.push(self.to_index(coords.0, coords.1 + 1, size));
        }

        surrounding
    }

    fn get_coords(&self, size: (usize, usize)) -> (usize, usize) {
        (self.id / size.1, self.id % size.1)
    }
    fn to_index(&self, y: usize, x: usize, size: (usize, usize)) -> usize {
        x + y * size.1
    }
    fn is_lowest(&self, field: &Vec<Coord>, size: (usize, usize)) -> bool {
        let surrounding = self.get_surrounding(size);
        for index in surrounding {
            if field[index].val <= self.val {
                return false;
            }
        }
        true
    }
}


impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lowest_point {
            write!(f, "{}", self.val.to_string().bright_yellow())
        } else if self.basin {
            match self.basin_id % 5 {
                0 => write!(f, "{}", self.val.to_string().green()),
                1 => write!(f, "{}", self.val.to_string().magenta()),
                2 => write!(f, "{}", self.val.to_string().cyan()),
                3 => write!(f, "{}", self.val.to_string().red()),
                4 => write!(f, "{}", self.val.to_string().yellow()),
                _ => write!(f, "{}", self.val.to_string().purple()),
            }
        } else {
            write!(f, "{}", self.val)
        }
    }
}

fn main() {
    println!("\n{}", "AOC Day 9".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day9/testInput.txt" } else { "src/day9/input.txt" });

    let (mut field, size) = create_field(lines);

    let field_ref = field.clone();
    let field: Vec<Coord> = field.iter_mut().map(|coord| coord.update(&field_ref, size)).collect();

    let lowest_points = field.iter().filter(|coord| coord.lowest_point == true).collect::<Vec<&Coord>>();
    let sum = lowest_points.iter().fold(0, |acc, coord| acc + coord.val + 1);

    part1(sum);
    part2(field.to_owned(), lowest_points, size);
}

fn part1(sum: usize) {
    println!("{}", "Part 1".green());
    println!("- {}", sum.to_string().yellow());
}

fn part2(field: Vec<Coord>, lowest_points: Vec<&Coord>, size: (usize, usize)) {
    let mut field = field.clone();
    let mut largest = [0;3];

    for low_point in lowest_points {
        let mut basin_size = 0;
        let basin_id = low_point.id;
        let mut frontier = Vec::<usize>::new();
        frontier.push(low_point.clone().id);

        // Flood fill
        while frontier.len() > 0 {
            let id = frontier.pop().unwrap();
            let coord = field.get_mut(id).unwrap();
            if coord.basin {
                continue;
            }
            basin_size += 1;
            coord.basin = true;
            coord.basin_id = basin_id;
            let surrounding = coord.get_surrounding(size);

            for index in surrounding {
                if !field[index].basin && field[index].val < 9 {
                    frontier.push(index);
                }
            }
        }

        let smallest = largest.iter().min().unwrap();
        if basin_size.ge(smallest) {
            let pos = largest.iter().position(|val| val.eq(smallest)).unwrap();
            largest[pos] = basin_size.clone();
        }
    }

    let answer = largest.iter().fold(1, |acc, x| acc * x);
    println!("{:?}", largest);
    println!("{}", "Part 2".green());
    print::grid(&field, size);
    println!("- {}", answer.to_string().yellow());
}

fn create_field(lines: Vec<String>) -> (Vec<Coord>, (usize, usize)) {
    let mut field: Vec<Coord> = Vec::new();
    // size, y, x.
    let size: (usize, usize) = (lines.len(), lines[0].len());

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            field.push(Coord::from(x + y * size.1, lines[y].chars().nth(x).unwrap()));
        }
    }

    (field, size)
}