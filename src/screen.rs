use std::io::{Stdout, stdout};

use crossterm::{
    ExecutableCommand as _,
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};

use crate::interface::InterfaceState;

pub struct Screen {
    pub standard_output_stream: Stdout,
}

impl Screen {
    #[expect(clippy::unwrap_used, reason = "App in useless state if setup fails.")]
    pub fn new() -> Self {
        let mut stdout = stdout();

        stdout
            .execute(EnterAlternateScreen)
            .unwrap()
            .execute(SetTitle("dme"))
            .unwrap()
            .execute(Hide)
            .unwrap();

        Self {
            standard_output_stream: stdout,
        }
    }

    pub const fn update(&self, _interface_state: Option<&InterfaceState>) {}
}

impl Drop for Screen {
    #[expect(clippy::unwrap_used, reason = "App in useless state if drop fails.")]
    fn drop(&mut self) {
        self.standard_output_stream
            .execute(LeaveAlternateScreen)
            .unwrap()
            .execute(Show)
            .unwrap();
    }
}
