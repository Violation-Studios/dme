use std::io::{Stdout, stdout};

use crossterm::{
    ExecutableCommand as _,
    event::{
        DisableMouseCapture, EnableMouseCapture, Event,
        KeyCode::{self},
        read,
    },
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::interface::InterfaceCommand;

pub struct Input {
    pub standard_output_stream: Stdout,
}

impl Input {
    #[expect(clippy::unwrap_used, reason = "App in useless state if setup fails.")]
    pub fn new() -> Self {
        let mut stdout = stdout();

        stdout.execute(EnableMouseCapture).unwrap();
        enable_raw_mode().unwrap();

        Self {
            standard_output_stream: stdout,
        }
    }

    #[expect(clippy::panic, reason = "don't care atm, pre-alpha temp :shrug")]
    #[expect(clippy::unwrap_used, reason = "don't care atm, pre-alpha temp :shrug")]
    pub fn get_command() -> InterfaceCommand {
        loop {
            let Event::Key(key_event) = read().unwrap() else {
                continue;
            };

            return match key_event.code {
                KeyCode::Enter => InterfaceCommand::ActivateSelection,
                KeyCode::Left => InterfaceCommand::MoveSelectionLeftwards,
                KeyCode::Right => InterfaceCommand::MoveSelectionRightwards,
                KeyCode::Up => InterfaceCommand::MoveSelectionUpwards,
                KeyCode::Down => InterfaceCommand::MoveSelectionDownwards,
                KeyCode::Esc => panic!(),
                _ => continue,
            };
        }
    }
}

impl Drop for Input {
    #[expect(clippy::unwrap_used, reason = "App in useless state if drop fails.")]
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        self.standard_output_stream
            .execute(DisableMouseCapture)
            .unwrap();
    }
}
