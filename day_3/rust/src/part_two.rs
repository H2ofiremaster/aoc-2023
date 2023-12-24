use std::collections::HashSet;
use std::fs;
use std::fmt::Debug;
use grid::*;

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    adjacent_chars: Vec<Symbol>,
}

impl Number {
    fn new(value: u32, adjacent_chars: Vec<Symbol>) -> Self {
        Self {
            value,
            adjacent_chars,
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
                    current_number.parse().expect("'current_number' should be a number"),
                    get_adjacent_symbols(grid, index - char_count, char_count),
                ));

                current_number = String::new();
            } else {
                current_number.push(*character);
            }
        }
        result
    }

    fn contains_char(&self, value: char) -> bool {
        self.adjacent_chars.iter().any(|s| s.value == value)
    }
}


#[derive(Debug, Clone)]
struct Engine {
    numbers: Vec<Number>,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Symbol {
    value: char,
    location: (usize, usize)
}

struct Gear {
    numbers: [Number; 2],
}

impl Gear {
    fn get_ratio(&self) -> u32 {
        self.numbers[0].value * self.numbers[1].value
    }
}


const FILE_NAME: &str = "input.txt";


fn get_adjacent_symbols(grid: &Grid<char>, start_index: usize, digits: usize) -> Vec<Symbol> {
    let start_index: isize = start_index as isize;
    let end_index: isize = start_index + (digits as isize);
    let mut adjecent_chars: Vec<Option<Symbol>> = Vec::new();
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
    adjecent_chars.into_iter()
    .flatten()
    .filter(|c| !matches!(c.value, '.' | '\n'))
    .collect()
}

fn get_grid_element(grid: &Grid<char>, row: isize, col: isize) -> Option<Symbol> {
    if row < 0 || col < 0 { return None; }
    let row: usize = row.try_into().expect("row should not be less than 0");
    let col: usize = col.try_into().expect("col should not be less than 0");
    return grid.get(row, col).map(|value| Symbol {
        value: *value,
        location: (row, col)
    });
}

fn main() {


    let input: String = fs::read_to_string(FILE_NAME)
        .expect("input file should exist")
        .trim_end().replace('\r', "") + "\n";
    let engine: Engine = Engine::new(&input);

    let filtered_engine: Engine = Engine {
        numbers: engine.numbers.into_iter()
        .filter(|number| number.contains_char('*'))
        .map(|number| {
            let filtered_chars: Vec<Symbol> = number.adjacent_chars.into_iter()
            .filter(|symbol| symbol.value == '*')
            .collect();
            Number::new(number.value, filtered_chars)
        }).collect()
    };

    let gears: Vec<Gear> = filtered_engine.numbers.iter()
        .flat_map(|number| number.adjacent_chars.to_vec())
        .collect::<HashSet<Symbol>>().iter()
        .filter_map(|symbol| {
            let numbers: Vec<Number> = filtered_engine.numbers
                .iter()
                .filter(|num| num.adjacent_chars.contains(symbol))
                    .cloned()
                    .collect();
            if numbers.len() != 2 { None }
            else { 
                Some(Gear {
                    numbers: numbers.try_into()
                        .expect("vec should not have more than two numbers") 
                }) 
            }
        }).collect();
    
    let output: u32 = gears.iter().map(Gear::get_ratio).sum();

    println!("Output: {}", output)

}
