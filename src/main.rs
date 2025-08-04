use crate::{
    game::Game,
    input::Input,
    interface::{Interface, InterfaceState},
    screen::Screen,
};

mod game;
mod input;
mod interface;
mod screen;

fn main() -> ! {
    let app = App {
        input: Input::new(),
        interface: Interface::default(),
        game: Game,
        screen: Screen::new(),
    };

    loop {
        app.update();
    }
}

/*
Load GameState from file or reuse in memory GameState

Output GameState & InterfaceState to user: video, audio, haptics

Wait for Input: keyboard, gamepad, mouse, microphone

Alter InterfaceState based on Input

Alter GameState based on InterfaceState/GameState

Alter InterfaceState based on GameState

Save GameState to file
*/

struct App {
    input: Input,
    interface: Interface,
    game: Game,
    screen: Screen,
}

impl App {
    fn update(&self) {
        let interface_command = Input::get_command();

        let game_command = self.interface.get_command(&interface_command);

        let game_state = self.game.update(game_command);

        let interface_state = self.interface.update(game_state, &interface_command);

        self.screen.update(interface_state);
    }
}
