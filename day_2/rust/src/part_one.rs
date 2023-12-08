
#[derive(PartialEq)]
enum CubeColor {
    Red, Green, Blue
}


struct GameRound {
    id: u32,
    cubes: Vec<CubeSet>,
}

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

impl TryFrom<&str> for CubeSet {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // String format: '<count> <color>'
        let mut parts = value.split(" ");
        
        let count: u32 = parts.next()
            .and_then(|string| string.parse().ok())
            .ok_or("'count' could not be initialized.")?;
        let color: CubeColor = parts.next()
            .and_then(|string| CubeColor::try_from(string).ok())
            .ok_or("'color' could not be initialized.")?;

        let output: CubeSet = CubeSet {
            count,
            color
        };
        return Ok(output)
    }
}

impl Solution {
    /*fn is_possible(&self, game: Game) -> bool {
        return game.rounds.iter()
            .
    }*/
}



const INPUT_SOLUTION: Solution = Solution {
    red_cubes: 12,
    green_cubes: 13,
    blue_cubes: 14,
};

fn main() {
    
}
