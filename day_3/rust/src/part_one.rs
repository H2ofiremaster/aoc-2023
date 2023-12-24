use std::fs;
use std::fmt::Debug;
use grid::*;

#[derive(Debug)]
struct Number {
    value: u32,
    adjacent_chars: Vec<char>,
}

#[derive(Debug)]
struct Engine {
    numbers: Vec<Number>,
}

impl Number {
    fn new(adjacent_chars: Vec<char>, value: u32) -> Self {
        Self {
            value,
            adjacent_chars
        }
    }

    fn from_char_grid(grid: &Grid<char>) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();
        let mut current_number: String = String::new();
        for (index, character) in grid.iter().enumerate() {
            if !character.is_numeric() {
                let char_count: usize = current_number.chars().count();
                if char_count == 0 { continue; }
                result.push(Number::new(
                    get_adjacent_chars(grid, index - char_count, char_count),
                    current_number.parse().expect("'current_number' should be a number")
                ));

                current_number = String::new();
            } else {
                current_number.push(*character);
            }
        }
        result
    }
}

impl Engine {
    fn new(input: &str) -> Self {
        let row = input.lines().next()
            .expect("input.lines() should not be empty")
            .chars();
        let layout: Grid<char> = Grid::from_vec(input.chars().collect(), row.count() + 1);
        let numbers: Vec<Number> = Number::from_char_grid(&layout);

        Engine { numbers }
    }
}

const FILE_NAME: &str = "input.txt";



fn get_adjacent_chars(grid: &Grid<char>, start_index: usize, digits: usize) -> Vec<char> {
    let start_index: isize = start_index as isize;
    let end_index: isize = start_index + (digits as isize);
    let mut adjecent_chars: Vec<Option<&char>> = Vec::new();
    for i in start_index..end_index {
        let cols: isize = grid.cols() as isize;
        let row: isize = i / cols;
        let col: isize = i % cols;

        let mut push_char = |row_offset: isize, col_offset: isize| 
            adjecent_chars.push(get_grid_element(grid, row + row_offset, col + col_offset));
        
        push_char(-1, 0);
        push_char(1, 0);
        if i == start_index {
            push_char(0, -1);
            push_char(-1, -1);
            push_char(1, -1);
        }
        if i == (end_index - 1) {
            push_char(0, 1);
            push_char(-1, 1);
            push_char(1, 1);
        }
    }
    adjecent_chars.iter()
    .filter(|c| c.is_some())
    .map(|c| *c.unwrap())
    .filter(|c| !matches!(*c, '.' | '\n'))
    .collect()
}

fn get_grid_element<T: Debug>(grid: &Grid<T>, row: isize, col: isize) -> Option<&T> {
    if row < 0 || col < 0 { return None; }
    let r: usize = row.try_into().expect("row should not be less than 0");
    let c: usize = col.try_into().expect("col should not be less than 0");
    return grid.get(r, c);
}

fn main() {


    let input: String = fs::read_to_string(FILE_NAME)
        .expect("input file should exist")
        .trim_end().replace('\r', "") + "\n";
    let engine: Engine = Engine::new(&input);
    let output: u32 = engine.numbers.iter()
    .filter(|number| !number.adjacent_chars.is_empty())
    .map(|number| number.value)
    .sum();

    println!("Output: {}", output)
}
