use bevy::{app::ScheduleRunnerPlugin, input::InputPlugin, prelude::*};
use core::time::Duration;

mod ui;
use crate::ui::terminal::Crossterm as Terminal;
use crate::ui::terminal::Position;
use crate::ui::terminal::QuantifiedEntry;
use crate::ui::terminal::Terminal as _;
use crate::ui::terminal::UIElement as _;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                0.166,
            ))),
            InputPlugin,
            Terminal::new(),
        ))
        .run();
}

#[derive(Resource)]
pub struct WoodCount(u8);

impl WoodCount {
    pub const fn increment(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}

#[expect(
    clippy::unwrap_used,
    reason = "Game should crash if terminal setup fails"
)]
fn setup_terminal(mut terminal: ResMut<Terminal>) {
    terminal.setup().unwrap();
}

#[expect(clippy::needless_pass_by_value, reason = "Bevy convention")]
fn print_wood_count(wood_count: Res<WoodCount>, mut terminal: ResMut<Terminal>) {
    let wood_count_display =
        QuantifiedEntry::new("wood", wood_count.0, Position { x: 2u16, y: 2u16 });
    wood_count_display
        .draw(terminal.output_buffer())
        .unwrap_or(());
}
