use std::{fs, collections::HashMap};
use onig::*;

const INPUT_FILE: &str = "input.txt";
fn main() {
    let number_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    let text: String = fs::read_to_string(INPUT_FILE)
        .unwrap_or_else(|_| panic!("file path '{}' should be a valid text file", INPUT_FILE));

    //println!("Text:\n{}\n", text);
    
    let result: u32 = text.lines()
        .map(|line| get_numbers(line, &number_map))
        .map(|line| get_edge_chars(line.as_str()))
        .map(|(first, last)| first * 10 + last).sum();
    
    println!("Result: {}", result)
}

fn get_numbers(input: &str, number_map: &HashMap<&str, &str>) -> String {
    let numbers_regex_string: &str = &format!("(?=(1{}))", 
    number_map.iter()
    .fold("".to_owned(), |acc, (k, v)| format!("{}|{}|{}", acc, k, v)));

    return Regex::new(numbers_regex_string)
        .expect(&format!("'{}' should be a valid regex", numbers_regex_string))
        .captures_iter(input)
        .map(|c| c.at(1))
        .filter(|o| o.is_some())
        .map(|m| m.unwrap())
        .fold(String::new(), |accumulator, match_string| {
            if match_string.len() == 1 {
                return accumulator + match_string
            } else {
                return accumulator + number_map.get(match_string).unwrap_or(&match_string);
            }
        });


}

fn get_edge_chars(input: &str) -> (u32, u32) {
    if input.chars().count() < 1 
        { return (0, 0); }
    
    let mut chars = input.chars();
    let first_char: char = chars.next().unwrap();
    let last_char: char = chars.last().unwrap_or(first_char.clone());

    //println!("Chars: '{}', '{}'", first_char, last_char);

    return (first_char.to_digit(10).unwrap_or(0), 
        last_char.to_digit(10).unwrap_or(0));
}