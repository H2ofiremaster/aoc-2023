
#[derive(PartialEq)]
enum CubeColor {
    Red, Green, Blue
}


struct GameRound {
    id: u32,
    cubes: Vec<CubeSet>
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


impl GameRound {
    fn get_max_cubes(&self, color: CubeColor) -> u32 {
        return self.cubes.iter()
            .filter(|cube| cube.color == color)
            .map(|cube| cube.count)
            .max().expect("iterator should not be empty");
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
