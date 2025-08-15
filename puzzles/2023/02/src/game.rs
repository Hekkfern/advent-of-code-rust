type GameId = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct GameRound {
    pub num_green_balls: u32,
    pub num_red_balls: u32,
    pub num_blue_balls: u32,
}

impl GameRound {
    pub fn new(green: u32, red: u32, blue: u32) -> Self {
        Self {
            num_green_balls: green,
            num_red_balls: red,
            num_blue_balls: blue,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: GameId,
    pub rounds: Vec<GameRound>,
}

impl Game {
    pub fn new(id: GameId, rounds: Vec<GameRound>) -> Self {
        Self { id, rounds }
    }
}
