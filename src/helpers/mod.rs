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
}