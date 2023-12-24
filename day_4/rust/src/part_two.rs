use std::{fs, num::ParseIntError, fmt::Display};


enum CardParseError {
    ColonSplitTooShort(usize, usize),
    PipeSplitTooShort(usize, usize),
    NotNumber(ParseIntError),

}

impl Display for CardParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardParseError::ColonSplitTooShort(error_index, expected_length) =>
                write!(f, "Could not parse card; encountered 'None' at index {error_index} of colon_split. Expected length: {expected_length}"),
            CardParseError::PipeSplitTooShort(error_index, expected_length) =>
                write!(f, "Could not parse card; encountered 'None' at index {error_index} of pipe_split. Expected length: {expected_length}"),
            CardParseError::NotNumber(error) =>
                write!(f, "Could not parse card; encountered an error parsing an integer: {error}"),
        }
    }
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Result<Self, CardParseError> {
        let mut colon_splt = line.split(':');
        let id: u32 = colon_splt.next()
            .ok_or(CardParseError::ColonSplitTooShort(0, 2))?
            .replace("Card", "").replace(' ', "")
            .parse().map_err(CardParseError::NotNumber)?;

        let mut pipe_split = colon_splt.next()
            .ok_or(CardParseError::ColonSplitTooShort(0, 1))?
            .split('|');

        let winning_numbers: Vec<u32> = parse_numbers(pipe_split.next()
            .ok_or(CardParseError::PipeSplitTooShort(0, 2))?);
        
        let your_numbers: Vec<u32> = parse_numbers(pipe_split.next()
            .ok_or(CardParseError::PipeSplitTooShort(1, 2))?);

        Ok(Card { 
            id, 
            winning_numbers, 
            your_numbers,
        })
    }

    fn get_matching_numbers(&self) -> Vec<u32> {
        self.your_numbers.iter()
            .filter(|x| self.winning_numbers.contains(x))
            .copied()
            .collect()
    }
}

fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .filter_map(|n| n.parse::<u32>()
            .map_err(|err| println!("Number '{n}' failed to parse: {err}"))
            .ok())
        .collect()
}

fn increment_stacks(stacks: &mut [u32]) {
    if let Some((amount, stacks)) = stacks.split_first_mut() {

        for stack in stacks {
            *stack += *amount;
        }
    } else {
        println!("slice is empty")
    }
    
}

const INPUT_PATH: &str = "./input.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH)
        .expect("input file should exist");

    let cards: Vec<Card> = input.lines()
        .filter_map(|line| Card::new(line)
            .map_err(|err| println!("Card '{line}' failed to parse: {err}"))
            .ok())
        .collect();

    let mut card_counts: Vec<u32> = vec![1; cards.len() + 1];
    card_counts[0] = 0;

    for card in &cards {
        let matching_numbers: u32 = card.id + (card.get_matching_numbers().len() as u32);
        let slice: &mut [u32] = &mut card_counts[card.id as usize..=matching_numbers as usize];
        increment_stacks(slice);
    }
    
    let output: u32 = card_counts.iter().sum();

    println!("Output: {output}")
}
