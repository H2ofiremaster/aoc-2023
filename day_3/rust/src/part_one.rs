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
    fn new(grid: &Grid<char>, start_index: usize, value: u32) -> Self {
        let start_index: isize = start_index as isize;
        let value_digits: isize = (value.checked_ilog10().unwrap_or(0) + 1) as isize;
        let end_index: isize = (start_index as isize) + value_digits;
        //dbg!(&end_index);
        let mut adjecent_chars: Vec<Option<&char>> = Vec::new();
        for i in start_index..end_index {
            let cols: isize = grid.cols() as isize;
            let row: isize = i / cols;
            let col: isize = i % cols;
            //println!("i: {}, cols: {} row: {}, col: {}", i, cols, row, col);

            adjecent_chars.push(get_grid_element(grid, row - 1, col));
            adjecent_chars.push(get_grid_element(grid, row + 1, col));
            if i == start_index {
                adjecent_chars.push(get_grid_element(grid, row, col - 1));
                adjecent_chars.push(get_grid_element(grid, row - 1, col - 1));
                adjecent_chars.push(get_grid_element(grid, row + 1, col - 1));
            }
            if i == (end_index - 1) {
                adjecent_chars.push(get_grid_element(grid, row, col + 1));
                adjecent_chars.push(get_grid_element(grid, row - 1, col + 1));
                adjecent_chars.push(get_grid_element(grid, row + 1, col + 1));
            }
        }
        let adjacent_chars: Vec<char> = adjecent_chars.iter()
            .filter(|c| c.is_some())
            .map(|c| *c.unwrap())
            .filter(|c| c != &'.' && c != &'\n')
            .collect();

        if !adjacent_chars.is_empty() {
            println!("{:?} --- {:?}", value, adjacent_chars)
        }

        return Self {
            value,
            adjacent_chars
        };
    }

    fn from_char_grid(grid: &Grid<char>) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();
        let mut current_number: String = "".to_string();
        for (index, character) in grid.iter().enumerate() {
            if !character.is_numeric() {
                if char_count(&current_number) == 0 { continue; }

                let start_index: usize = index - char_count(&current_number);
                //dbg!(&current_number);



                let number: Number = Number::new(grid, start_index, current_number.parse().expect("'current_number' should be a number"));
                result.push(number);
                current_number = "".to_string();
            } else {
                current_number.push(*character);
            }
        }
        return result;
    }
}

impl Engine {
    fn new(input: &str) -> Self {
        let row = input.lines().next()
            .expect("input.lines() should not be empty")
            .chars();
        println!("{:?}", row);
        let row_length = row.count() + 1;
        println!("{:?}", row_length);
        let contents: Vec<char> = input.chars().collect();
        let layout: Grid<char> = Grid::from_vec(contents, row_length);
        let numbers: Vec<Number> = Number::from_char_grid(&layout);

        return Engine {
            numbers,
        };
    }
}

const FILE_NAME: &str = "input.txt";

fn char_count(string: &str) -> usize {
    string.chars().count()
}

fn get_grid_element<T: Debug>(grid: &Grid<T>, row: isize, col: isize) -> Option<&T> {
    //println!("T: {}, {}", row, col);
    if row < 0 || col < 0 { return None; }
    let r: usize = row.try_into().expect("row should not be less than 0");
    let c: usize = col.try_into().expect("col should not be less than 0");
    return grid.get(r, c);
}

fn main() {


    let input: String = fs::read_to_string(FILE_NAME)
        .expect("input file should exist")
        .trim_end().replace("\r", "") + "\n";
    let engine: Engine = Engine::new(&input);
    //dbg!(&engine);
    let output: u32 = engine.numbers.iter()
    .filter(|number| !number.adjacent_chars.is_empty())
    .map(|number| number.value)
    .sum();

    println!("Output: {}", output)
}
