use Advent_Of_Code_2021::read_in;
use colored::Colorize;
use Advent_Of_Code_2021::helpers::print;

const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 11".bright_green().bold());
    let (grid, size) = read_in::parse_grid(if TEST_MODE { "src/day11/testInput.txt" } else { "src/day11/input.txt" });

    part1(&grid, &size);
    part2(&grid, &size);
}

fn part1(grid: &Vec<usize>, size: &(usize, usize)) {
    let mut stepped_grid = grid.clone();
    let mut flashes = 0;

    for _i in 0..100 {
        let (new_grid, flashes_occurred) = step(&stepped_grid, size);
        stepped_grid = new_grid;
        flashes += flashes_occurred;
    }

    println!("{}", "Part 1".green());
    print::grid_special(&stepped_grid, size, move |x| {*x==0});
    println!("- {} Flashes", flashes.to_string().yellow());
}

fn part2(grid: &Vec<usize>, size: &(usize, usize)) {
    let mut stepped_grid = grid.clone();
    let mut synchronized_on = 0;
    for i in 0..1000 {
        let (new_grid, flashes_occurred) = step(&stepped_grid, size);
        stepped_grid = new_grid;
        if flashes_occurred == size.1 * size.0 {
            synchronized_on = i+1;
            break
        }
    }
    // print::grid_special(&stepped_grid, size, move |x| {*x==0});

    println!("{}", "Part 2".green());
    println!("- {} steps to sync", synchronized_on.to_string().yellow());
}

fn step(grid: &Vec<usize>, size: &(usize, usize)) -> (Vec<usize>, usize) {
    // Increase all energy levels by 1
    let mut increased: Vec<_> = grid.iter().map(|val| val + 1).collect();

    // Flash
    let mut flashes = 0;
    let mut has_updated = true;
    while has_updated {
        has_updated = false;
        for y in 0..size.1 {
            for x in 0..size.0 {
                if increased[index(y, x, size)] <= 9 { continue; }
                increased[index(y, x, size)] = 0;
                has_updated = true;
                flashes += 1;
                let neighbours = get_neighbours(y, x, size);

                for neighbour in neighbours {
                    if increased[neighbour] == 0 || increased[neighbour] > 9 { continue; }
                    increased[neighbour] += 1;
                }
            }
        }
    }

    (increased, flashes)
}

fn get_neighbours(y: usize, x: usize, size: &(usize, usize)) -> Vec<usize> {
    let mut neighbours = Vec::new();
    for y_offset in -1..=1 {
        for x_offset in -1..=1 {
            let (new_y, new_x) = (y as i32 + y_offset, x as i32 + x_offset);
            if new_y >= 0 && new_y < size.1 as i32 && new_x >= 0 && new_x < size.0 as i32 {
                neighbours.push(new_y as usize * size.0 + new_x as usize);
            }
        }
    }
    neighbours
}

fn index(y: usize, x: usize, size: &(usize, usize)) -> usize {
    x + y * size.0
}