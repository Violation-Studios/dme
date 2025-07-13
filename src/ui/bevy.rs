use bevy::prelude::*;
use core::time::Duration;
use crossterm::event::{Event, KeyCode::Char, poll, read};

use crate::{WoodCount, ui::terminal::Crossterm};
use crate::{print_wood_count, setup_terminal, ui::terminal::Terminal as _};

impl Plugin for Crossterm {
    fn build(&self, app: &mut App) {
        app.insert_resource(Self::new())
            .insert_resource(WoodCount(0))
            .add_systems(Startup, setup_terminal)
            .add_systems(
                Update,
                (print_wood_count, crossterm_input_update_wood_count),
            );
    }
}

fn crossterm_input_update_wood_count(mut wood_count: ResMut<WoodCount>) {
    poll(Duration::from_secs_f64(0.083))
        .ok()
        .filter(|&ready| ready)
        .is_some()
        .then(|| read().ok())
        .is_some_and(
            |event| matches!(event, Some(Event::Key(key_event)) if key_event.code == Char(' ')),
        )
        .then(|| wood_count.increment());
}
