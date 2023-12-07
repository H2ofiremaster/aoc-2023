use std::fs;
use regex::Regex;


const INPUT_FILE: &str = "input.txt";
fn main() {
    let regex: Regex = Regex::new("[^0-9\\n]").unwrap();
    
    let text: String = fs::read_to_string(INPUT_FILE)
        .map(|text| regex.replace_all(text.as_ref(), "").to_string())
        .unwrap_or_else(|_| panic!("file path '{}' was not a valid text file.", INPUT_FILE));

    //println!("Text:\n{}\n", text);
    
    let result: u32 = text.lines()
        .map(|line| get_edge_chars(line))
        .map(|(first, last)| first * 10 + last).sum();
    
    println!("Result: {}", result)
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