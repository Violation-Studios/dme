use bevy::prelude::*;
use crossterm::{
    ExecutableCommand as _,
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, SetTitle, disable_raw_mode, enable_raw_mode,
    },
};
use std::io;

pub trait Terminal {
    fn new() -> Self;
    fn output_buffer(&mut self) -> &mut io::Stdout;
    fn setup(&mut self) -> io::Result<()>;
    fn teardown(&mut self) -> io::Result<()>;
}

#[derive(Resource)]
pub struct Crossterm {
    output_buffer: io::Stdout,
}

impl Terminal for Crossterm {
    fn new() -> Self {
        Self {
            output_buffer: io::stdout(),
        }
    }

    fn output_buffer(&mut self) -> &mut io::Stdout {
        &mut self.output_buffer
    }

    fn setup(&mut self) -> io::Result<()> {
        self.output_buffer()
            .execute(EnterAlternateScreen)?
            .execute(SetTitle("dme"))?
            .execute(Hide)?;

        enable_raw_mode()?;
        Ok(())
    }

    fn teardown(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        self.output_buffer()
            .execute(LeaveAlternateScreen)?
            .execute(Show)?;
        Ok(())
    }
}

#[expect(
    clippy::unwrap_used,
    reason = "Program is ending regardless if the terminal is restored."
)]
impl Drop for Crossterm {
    fn drop(&mut self) {
        self.teardown().unwrap();
    }
}

pub trait UIElement {
    fn draw(&self, buffer: &io::Stdout) -> io::Result<()>;
}

pub struct QuantifiedEntry {
    name: String,
    quantity: u8,
    position: Position,
}

#[derive(Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl QuantifiedEntry {
    pub fn new(name: &str, quantity: u8, position: Position) -> Self {
        Self {
            name: name.to_owned(),
            quantity,
            position,
        }
    }
}

impl UIElement for QuantifiedEntry {
    fn draw(&self, mut buffer: &io::Stdout) -> io::Result<()> {
        let text = format!("{:<13}{}{:>3}", self.name, " ", self.quantity);
        let text_size = ContentSize::new(text.len(), 1usize);

        UIFrame::new(text_size, self.position.clone()).draw(buffer)?;

        execute!(
            buffer,
            MoveTo(
                self.position.x.saturating_add(1u16),
                self.position.y.saturating_add(1u16)
            ),
            Print(text),
        )?;

        Ok(())
    }
}

struct UIFrame {
    content_size: ContentSize,
    position: Position,
}

impl UIElement for UIFrame {
    fn draw(&self, buffer: &io::Stdout) -> io::Result<()> {
        self.draw_top(buffer)?;
        self.draw_middle(buffer)?;
        self.draw_bottom(buffer)?;

        Ok(())
    }
}

impl UIFrame {
    const fn new(content_size: ContentSize, position: Position) -> Self {
        Self {
            content_size,
            position,
        }
    }
    fn draw_top(&self, mut buffer: &io::Stdout) -> io::Result<()> {
        let mut top = String::new();
        top.push_str(&FrameCharacter::CornerTopLeft.character());
        top.push_str(
            &FrameCharacter::Horizontal
                .character()
                .repeat(self.content_size.width),
        );
        top.push_str(&FrameCharacter::CornerTopRight.character());

        execute!(buffer, MoveTo(self.position.x, self.position.y), Print(top),)
    }

    fn draw_middle(&self, mut buffer: &io::Stdout) -> io::Result<()> {
        let mut middle = String::new();
        middle.push_str(&FrameCharacter::Vertical.character());
        middle.push_str(&" ".repeat(self.content_size.width));
        middle.push_str(&FrameCharacter::Vertical.character());

        for line in 1..=self.content_size.height {
            execute!(
                buffer,
                MoveTo(self.position.x, self.position.y.saturating_add(line as u16)),
                Print(&middle),
            )?;
        }
        Ok(())
    }
    fn draw_bottom(&self, mut buffer: &io::Stdout) -> io::Result<()> {
        let mut bottom = String::new();
        bottom.push_str(&FrameCharacter::CornerBottomLeft.character());
        bottom.push_str(
            &FrameCharacter::Horizontal
                .character()
                .repeat(self.content_size.width),
        );
        bottom.push_str(&FrameCharacter::CornerBottomRight.character());

        execute!(
            buffer,
            MoveTo(
                self.position.x,
                self.position
                    .y
                    .saturating_add(1u16.saturating_add(self.content_size.height as u16))
            ),
            Print(bottom),
        )
    }
}

struct ContentSize {
    width: usize,
    height: usize,
}

impl ContentSize {
    const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

enum FrameCharacter {
    Horizontal,
    Vertical,
    CornerBottomLeft,
    CornerBottomRight,
    CornerTopLeft,
    CornerTopRight,
}

impl FrameCharacter {
    fn character(&self) -> String {
        match *self {
            Self::Horizontal => "━",
            Self::Vertical => "┃",
            Self::CornerBottomLeft => "┗",
            Self::CornerBottomRight => "┛",
            Self::CornerTopLeft => "┏",
            Self::CornerTopRight => "┓",
        }
        .to_owned()
    }
}
