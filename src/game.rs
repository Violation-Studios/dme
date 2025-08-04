pub enum GameCommand {}

pub struct Game;

impl Game {
    pub const fn update(&self, _game_command: Option<&GameCommand>) -> Option<&GameState> {
        None
    }
}

pub struct GameState;
