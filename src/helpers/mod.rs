pub mod point;

pub mod read_in {
    use std::fs;

    pub fn from(file_path: &str) -> String {
        fs::read_to_string(file_path).expect("Could not read file at path:")
    }
    pub fn split_parse(file_path: &str, delimiter: &str) -> Vec<i32> {
        from(file_path).split(&delimiter).map(|val| match val.parse::<i32>() {
            Ok(val) => val,
            Err(error) => panic!("Could not parse number from file at path:{}, with error: {}", file_path, error),
        }).collect()
    }

    pub fn lines(file_path: &str) -> Vec<String> {
        let input = from(file_path);
        input.lines().map(|x| x.trim().to_string()).collect()
    }

    pub fn parse_lines(file_path: &str) -> Vec<i32> {
        let lines = lines(file_path);
        let mut parsed: Vec<i32> = Vec::with_capacity(lines.len());

        for line in lines {
            let value = line.parse::<i32>().expect(&format!("could not parse {line}"));
            parsed.push(value);
        }
        parsed
    }

    /**
     * Parses the input to a 1D grid, the sizes of the grid are returned as well.
     * The grid values are parsed to usize.<br>
     * @<b>returns</b> (grid, size)
     * - <b>size</b>: (usize, usize) with (x, y)
     * - <b>grid</b>: Vec\<usize\> with grid\[x + y * size.0]
     */
    pub fn parse_grid(file_path: &str) -> (Vec<usize>, (usize, usize)) {
        let lines = lines(file_path);
        let size = (lines[0].len(), lines.len());
        let mut grid = Vec::with_capacity(size.0 * size.1);

        for line in lines {
            for char in line.chars() {
                grid.push(char.to_digit(10).unwrap() as usize);
            }
        }

        (grid.clone(), size)
    }
}

pub mod print {
    use colored::Colorize;

    pub fn debug_grid_2d<T: std::fmt::Debug>(field: Vec<Vec<T>>) {
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                print!("{:?}", field[y][x]);
            }
            println!();
        }
    }

    pub fn grid_2d<T: std::fmt::Display>(field: Vec<Vec<T>>) {
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                print!("{}", field[y][x]);
            }
            println!();
        }
    }

    pub fn grid<T: std::fmt::Display>(field: &Vec<T>, size: &(usize, usize)) {
        for y in 0..size.0 {
            for x in 0..size.1 {
                print!("{}", field[x + y * size.1]);
            }
            println!()
        }
    }

    pub fn grid_special<T: std::fmt::Display>(field: &Vec<T>, size: &(usize, usize), glow_if: fn(&T) -> bool) {
        for y in 0..size.0 {
            for x in 0..size.1 {
                let print_val = if glow_if(&field[x + y * size.1]) {field[x + y * size.1].to_string().bright_white()} else { field[x + y * size.1].to_string().white() };
                print!("{}", print_val);
            }
            println!()
        }
    }
}