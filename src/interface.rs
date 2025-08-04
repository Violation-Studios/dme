use crate::game::{GameCommand, GameState};

pub enum InterfaceCommand {
    MoveSelectionUpwards,
    MoveSelectionDownwards,
    MoveSelectionLeftwards,
    MoveSelectionRightwards,
    ActivateSelection,
}

#[derive(Default)]
pub struct Interface {
    elements: Vec<InterfaceElement>,
}

impl Interface {
    pub const fn get_command(&self, _interface_command: &InterfaceCommand) -> Option<&GameCommand> {
        None
    }
    pub const fn update(
        &self,
        _game_state: Option<&GameState>,
        _interface_command: &InterfaceCommand,
    ) -> Option<&InterfaceState> {
        None
    }
}

struct InterfaceElement;

pub struct InterfaceState;

pub struct Size {
    width: usize,
    height: usize,
}

impl Size {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    const fn with_offset(&self, width: isize, height: isize) -> Self {
        let width_offset = width.unsigned_abs();
        let height_offset = height.unsigned_abs();

        Self {
            width: if width < 0 {
                self.width.saturating_sub(width_offset)
            } else {
                self.width.saturating_add(width_offset)
            },
            height: if height < 0 {
                self.height.saturating_sub(height_offset)
            } else {
                self.height.saturating_add(height_offset)
            },
        }
    }

    pub const fn width(&self) -> &usize {
        &self.width
    }

    pub const fn height(&self) -> &usize {
        &self.height
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    const fn with_offset(&self, x: isize, y: isize) -> Self {
        let x_offset = x.unsigned_abs();
        let y_offset = y.unsigned_abs();

        Self {
            x: if x < 0 {
                self.x.saturating_sub(x_offset)
            } else {
                self.x.saturating_add(x_offset)
            },
            y: if y < 0 {
                self.y.saturating_sub(y_offset)
            } else {
                self.y.saturating_add(y_offset)
            },
        }
    }
}
