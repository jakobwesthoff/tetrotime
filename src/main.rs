mod digits;
mod tetromino;

use anyhow::Result;
use pixel_loop::canvas::{Canvas, RenderableCanvas};
use pixel_loop::canvas::{CrosstermCanvas, PixelsCanvas};
use pixel_loop::color::Color;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState, PixelsInputState};
use tetromino::DigitBoard;

struct State {
    board: DigitBoard,
    current_digit: usize,
}

impl State {
    fn new() -> Self {
        Self {
            board: DigitBoard::new(20),
            current_digit: 0,
        }
    }
}

fn main() -> Result<()> {
    let canvas = CrosstermCanvas::new();
    let input = CrosstermInputState::new();

    let state = State::new();

    eprintln!("Render size: {}x{}", canvas.width(), canvas.height());

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
    );
}
