use std::collections::{HashMap, HashSet};
use Advent_Of_Code_2021::read_in;
use colored::Colorize;
const TEST_MODE: bool = false;

fn main() {
    println!("\n{}", "AOC Day 8".bright_green().bold());
    let lines = read_in::lines(if TEST_MODE { "src/day8/testInput.txt" } else { "src/day8/input.txt" });

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    // How many times do segments of length: 2, 3, 4, 7 appear in the output
    let mut count = 0;
    for line in lines {
        let output = line.split(" | ").last().unwrap().trim();
        let segments = output.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();

        for segment in segments {
            let length = segment.len();
            if length == 2 || length == 3 || length == 4 || length == 7 {
                count += 1;
            }
        }
    }

    println!("{}", "Part 1".green());
    println!("- {}", count.to_string().yellow());
}

fn part2(lines: &Vec<String>) {
    println!("{}", "Part 2".green());
    let lines = lines.clone();
    let mut sum = 0;
    for line in lines {
        let (decoder, output) = decode(line);

        let mut number = 0;
        let segments = output.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();
        for i in 0..segments.len() {
            let set: HashSet<char> = hashset(&segments[i].chars().collect::<Vec<char>>());
            for ii in 0..decoder.len() {
                if set.eq(&decoder[ii]) {
                    number += ii as i32 * 10_i32.pow((segments.len() - i - 1) as u32);
                    break;
                }
            }
        }
        if TEST_MODE {
            println!("- {}", number.to_string().red().italic());
        }
        sum += number;
    }

    println!("- {}", sum.to_string().yellow());
}

fn decode(line: String) -> (Vec<HashSet<char>>, String) {
    let (input, output) = line.split_once(" | ").unwrap();
    let segments = input.split_whitespace().map(|x| x.trim().to_owned()).collect::<Vec<String>>();
    let mut map = HashMap::new();
    let mut length6 = Vec::new();
    let mut length5 = Vec::new();

    for segment in segments {
        let length = segment.len();
        let number = match length {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => -1
        };
        if number != -1 {
            map.insert(number, segment);
        } else if length == 6 {
            length6.push(segment);
        } else if length == 5 {
            length5.push(segment);
        }
    }

    let mut encoder = HashMap::new();

    let seven = map.get(&7).unwrap();
    let one = map.get(&1).unwrap();
    let four = map.get(&4).unwrap();
    let eight = map.get(&8).unwrap();

    // Top [a]
    encoder.insert("a", seven.chars().filter(|c| !one.contains(*c)).last().unwrap());

    // Right side, [c, f]
    let zero_is_f = length6.iter().all(|six_len| six_len.contains(one.chars().nth(0).unwrap()));
    if zero_is_f {
        encoder.insert("f", one.chars().nth(0).unwrap());
        encoder.insert("c", one.chars().nth(1).unwrap());
    } else {
        encoder.insert("c", one.chars().nth(0).unwrap());
        encoder.insert("f", one.chars().nth(1).unwrap());
    }

    // left top, middle [b, d]
    let b_d = four.chars().filter(|c| !one.contains(*c)).collect::<Vec<char>>();
    let first_is_d = length5.iter().all(|five| five.contains(b_d[0]));
    if first_is_d {
        encoder.insert("d", b_d[0]);
        encoder.insert("b", b_d[1]);
    } else {
        encoder.insert("d", b_d[1]);
        encoder.insert("b", b_d[0]);
    }

    let nine = length6.iter().filter(|seg| seg.contains(*encoder.get(&"d").unwrap()) && seg.contains(*encoder.get(&"c").unwrap())).last().unwrap();

    for character in nine.chars() {
        if encoder.values().all(|c| !c.eq(&character)) {
            encoder.insert(&"g", character);
        }
    }

    for character in eight.chars() {
        if encoder.values().all(|c| !c.eq(&character)) {
            encoder.insert(&"e", character);
        }
    }

    // print_segment(&encoder);

    // Create decoder
    let a = encoder.get("a").unwrap().to_owned();
    let b = encoder.get("b").unwrap().to_owned();
    let c = encoder.get("c").unwrap().to_owned();
    let d = encoder.get("d").unwrap().to_owned();
    let e = encoder.get("e").unwrap().to_owned();
    let f = encoder.get("f").unwrap().to_owned();
    let g = encoder.get("g").unwrap().to_owned();

    let mut decoder: Vec<HashSet<char>> = Vec::with_capacity(10);
    decoder.push(HashSet::from_iter([a, b, c, e, f, g]));    // 0
    decoder.push(HashSet::from_iter([c, f]));            // 1
    decoder.push(HashSet::from_iter([a, c, d, e, g]));      // 2
    decoder.push(HashSet::from_iter([a, c, d, f, g]));      // 3
    decoder.push(HashSet::from_iter([b, d, c, f]));        // 4
    decoder.push(HashSet::from_iter([a, b, d, f, g]));      // 5
    decoder.push(HashSet::from_iter([a, b, d, f, e, g]));    // 6
    decoder.push(HashSet::from_iter([a, c, f]));          // 7
    decoder.push(HashSet::from_iter([a, b, c, d, e, f, g]));  // 8
    decoder.push(HashSet::from_iter([a, b, c, d, f, g]));    // 9

    (decoder, output.to_owned())
}

fn print_segment(encoder: &HashMap<&str, char>) {
    let a = encoder.get("a").unwrap_or(&'•');
    let b = encoder.get("b").unwrap_or(&'•');
    let c = encoder.get("c").unwrap_or(&'•');
    let d = encoder.get("d").unwrap_or(&'•');
    let e = encoder.get("e").unwrap_or(&'•');
    let f = encoder.get("f").unwrap_or(&'•');
    let g = encoder.get("g").unwrap_or(&'•');
    println!("  {}{}{} ", a, a, a);
    println!(" {}   {} ", b, c);
    println!(" {}   {} ", b, c);
    println!("  {}{}{} ", d, d, d);
    println!(" {}   {} ", e, f);
    println!(" {}   {} ", e, f);
    println!("  {}{}{} ", g, g, g);
}

/*
 0: 6    1: 2    2: 5    3: 5    4: 4
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

 5: 5    6: 6    7: 3    8: 7    9: 6
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
 */

/*
   2: 5    3: 5    5: 5
   aaaa    aaaa    aaaa
  .    c  .    c  b    .
  .    c  .    c  b    .
   dddd    dddd    dddd
  e    .  .    f  .    f
  e    .  .    f  .    f
   gggg    gggg    gggg

  0: 6    6: 6    9: 6
  aaaa    aaaa    aaaa
 b    c  b    .  b    c
 b    c  b    .  b    c
  ....    dddd    dddd
 e    f  e    f  .    f
 e    f  e    f  .    f
  gggg    gggg    gggg

 1: 2   7: 3    4: 4    8: 7
 ....   aaaa   ....     aaaa
.    c .    c b    c   b    c
.    c .    c b    c   b    c
 ....   ....   dddd     dddd
.    f .    f .    f   e    f
.    f .    f .    f   e    f
 ....   ....   ....     gggg
 */

fn hashset(data: &[char]) -> HashSet<char> {
    HashSet::from_iter(data.iter().cloned())
}