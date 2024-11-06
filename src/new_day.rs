use std::fs;
use std::fs::{create_dir, File};
use std::io;
use std::io::Write;
use std::path::Path;
use colored::Colorize;

fn main() {
    // Retrieves the days from the existing folders
    let paths = fs::read_dir("./src").expect("Failed to read dir");
    let folders = paths.map(|path| { format!("{}", path.unwrap().path().display()) }).collect::<Vec<String>>();
    let folders = folders.iter().filter(|name| name.contains("day") && !name.ends_with(".rs")).collect::<Vec<&String>>();
    let days: Vec<i32> = folders.iter().map(|name| { name.replace("./src\\day", "").parse::<i32>().unwrap() }).collect::<Vec<i32>>();

    println!("Existing Days:");
    for day in days.clone() {
        println!("- Day: {}", day)
    }
    let latest_day = days.iter().max().unwrap();

    // Asks the user if they want to create that day
    loop {
        println!("Create day {}? [y/n] ", latest_day + 1);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim() {
            "y" | "yes" => break,
            "n" | "no" => return,
            _ => {}
        }
    }
    println!("Creating day..");
    create_day_files(latest_day + 1);
    update_cargo(days.clone(), latest_day + 1);
}

fn create_day_files(num: i32) {
    let folder_path = format!("./src/day{num}");
    create_dir(Path::new(&folder_path)).expect("Failed to create directory");
    let path_str = format!("{folder_path}/day{num}.rs");
    let path = Path::new(&path_str);
    let display = path.display();

    println!("Creating files");
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => panic!("Failed to create {display}, error {err}"),
    };
    println!("{} Created {}", "✓".green(), display);

    let init_code = INIT_CODE.replace("DAY_NUMBER", &num.to_string());
    match file.write_all(init_code.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("{} Populated {}", "✓".green(), display),
    }

    let path_str = format!("{folder_path}/input.txt");
    let path = Path::new(&path_str);
    match File::create(path) {
        Ok(_) => {  },
        Err(err) => panic!("Failed to create {display}, error {err}"),
    };
    println!("{} Created {}", "✓".green(), path.display());

    let path_str = format!("{folder_path}/testInput.txt");
    let path = Path::new(&path_str);
    match File::create(path) {
        Ok(_) => {  },
        Err(err) => panic!("Failed to create {display}, error {err}"),
    };
    println!("{} Created {}", "✓".green(), path.display());
}

fn update_cargo(mut days: Vec<i32>, num: i32) {
    println!("Updating Cargo.toml file");
    days.push(num);
    let mut bin_str = String::from("bin=[{name=\"new_day\", path=\"src/new_day.rs\"}, ");
    let template_str = "{name=\"dayDAY_NUM\", path=\"src/dayDAY_NUM/dayDAY_NUM.rs\"}, ";

    for day in days {
        let sub = template_str.replace("DAY_NUM", day.to_string().as_str());
        bin_str.push_str(&sub);
    }
    bin_str.push_str("]");

    let mut lines = fs::read_to_string("./Cargo.toml").expect("Could not read Cargo.toml")
        .lines().map(|s| s.to_string()).collect::<Vec<String>>();
    lines[0] = bin_str;
    let result = lines.join("\r\n");

    let mut cargo_file = File::create("./Cargo.toml").expect("Could access Cargo.toml");
    cargo_file.write_all(result.as_bytes()).expect("Could write to Cargo.toml");

    println!("{} Updated Cargo.toml", "✓".green());
}

static INIT_CODE: &str =
    "use Advent_Of_Code_2021::read_in;
const TEST_MODE: bool = false;

fn main() {
    println!(\"\\n{}\", \"AOC Day DAY_NUMBER\".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { \"src/dayDAY_NUMBER/testInput.txt\" } else { \"src/dayDAY_NUMBER/input.txt\" });

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let answer = \"None\";
    println!(\"{}\",\"Part 1\".green());
    println!(\"- {}\", answer.to_string().yellow());
}

fn part2(lines: &Vec<String>) {
    let answer = \"None\";
    println!(\"{}\",\"Part 2\".green());
    println!(\"- {}\", answer.to_string().yellow());
}";