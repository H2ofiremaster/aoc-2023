use std::fs;


#[derive(PartialEq, Debug)]
enum CubeColor {
    Red, Green, Blue
}


#[derive(Debug)]
struct GameRound {
    id: u32,
    cubes: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    count: u32,
    color: CubeColor,
}

struct Solution {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}


impl TryFrom<&str> for CubeColor {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err("Value was not a color.")
        }
    }
}

impl GameRound {
    fn get_max_cubes(&self, color: CubeColor) -> u32 {
        return self.cubes.iter()
            .filter(|cube| cube.color == color)
            .map(|cube| cube.count)
            .max().expect("iterator should not be empty");
    }
}

impl TryFrom<&str> for GameRound {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // String format: 'Game <id>: <CubeSet>, <CubeSet>; <CubeSet>
        let mut parts = value.split(":");

        let id: u32 = parts.next()
            .and_then(|string| string.replace("Game ", "").parse().ok())
            .ok_or("'id' could not be initialized.")?;
        let cubes: &str = parts.next()
            .ok_or("'cubes' could not be initialized.")?;
        let cubes: Vec<CubeSet> = cubes.split([',', ';'])
            .filter_map(|cube| CubeSet::try_from(cube).ok())
            .collect();

        Ok(GameRound {
            id,
            cubes,
        })
    }
}

impl TryFrom<&str> for CubeSet {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // String format: '<count> <color>'
        let mut parts = value.trim().split(" ");
        
        let count: u32 = parts.next()
            .and_then(|string| string.parse().ok())
            .ok_or("'count' could not be initialized.")?;
        let color: CubeColor = parts.next()
            .and_then(|string| CubeColor::try_from(string).ok())
            .ok_or("'color' could not be initialized.")?;

        Ok(CubeSet {
            count,
            color
        })
    }
}

impl Solution {
    fn is_possible_with(&self, game: &GameRound) -> bool {
        return self.red_cubes >= game.get_max_cubes(CubeColor::Red) && 
            self.green_cubes >= game.get_max_cubes(CubeColor::Green) &&
            self.blue_cubes >= game.get_max_cubes(CubeColor::Blue);
    }
}


const INPUT_SOLUTION: Solution = Solution {
    red_cubes: 12,
    green_cubes: 13,
    blue_cubes: 14,
};

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_FILE_NAME)
        .expect(&format!("{} should be a valid file name", INPUT_FILE_NAME));

    let rounds: Vec<GameRound> = input.lines()
        .filter_map(|line| {
            let round = GameRound::try_from(line);
            match round {
                Ok(r) => return Some(r),
                Err(e) => {
                    print!("Error: {}", e);
                    return None;
                },
            }
        }).collect();
    
    let output: u32 = rounds.iter().filter(|round| INPUT_SOLUTION.is_possible_with(round)).map(|round| round.id).sum();
    println!("Output: {}", output)
}