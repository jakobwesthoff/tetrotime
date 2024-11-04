mod digits;
mod tetromino;

use anyhow::Result;
use digits::Digit;
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::canvas::{Canvas, RenderableCanvas};
use pixel_loop::color::Color;
use pixel_loop::crossterm::terminal;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use pixel_loop::rand::Rng;
use tetromino::{Board, DigitBoard};

struct State {
    board: DigitBoard,
    current_digit: usize,
}

impl State {
    fn new(width: u32, height: u32) -> Self {
        Self {
            board: DigitBoard::new(20),
            current_digit: 0,
        }
    }
}

fn main() -> Result<()> {
    let (terminal_width, terminal_height) = terminal::size()?;
    let width = terminal_width;
    let height = terminal_height * 2;

    let mut canvas = CrosstermCanvas::new(width, height);
    canvas.set_refresh_limit(120);

    let state = State::new(width as u32, height as u32);
    let input = CrosstermInputState::new();

    eprintln!("Render size: {width}x{height}");

    pixel_loop::run(
        30,
        state,
        input,
        canvas,
        |e, s, input, canvas| {
            let width = canvas.width();
            let height = canvas.height();

            if input.is_key_pressed(KeyboardKey::Q) {
                std::process::exit(0);
            }

            if input.is_key_pressed(KeyboardKey::Space) {
                s.current_digit += 1;
                if s.current_digit > 9 {
                    s.current_digit = 0;
                }

                s.board.set_digit(s.current_digit.into());
            }

            s.board.update(canvas);

            Ok(())
        },
        |e, s, i, canvas, dt| {
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            s.board.render(canvas);

            canvas.render()?;

            Ok(())
        },
    )?;
    Ok(())
}
