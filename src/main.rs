mod digits;
mod tetromino;

use anyhow::Result;
use chrono::{DateTime, Duration, Local};
use digits::Digit;
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::canvas::{Canvas, RenderableCanvas};
use pixel_loop::color::Color;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use tetromino::DigitBoard;

fn time_to_digits(now: DateTime<Local>) -> Vec<Digit> {
    let now_string = now.format("%H%M%S").to_string();
    now_string
        .chars()
        .map(|c| Digit::from(c.to_digit(10).unwrap()))
        .collect()
}

struct State {
    boards: Vec<DigitBoard>,
    current_digits: Vec<Digit>,
    last_update_time: DateTime<Local>,
}

impl State {
    fn new() -> Self {
        Self {
            boards: vec![],
            current_digits: vec![],
            last_update_time: Local::now(),
        }
    }

    fn initialize_time(&mut self, width: u32, height: u32) {
        // Each digit is 6x10
        // Spacing 2 between each
        // There are 6 digits
        // -> height: 10
        // -> width: 6*6 + 5*2
        let x_start = (width as i64 - 6 * 6 - 5 * 2) / 2;
        let y_stop = (height as i64 + 10) / 2;
        let now = Local::now();
        let digits = time_to_digits(now);

        self.boards = digits
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, digit)| DigitBoard::new(x_start + (i as i64 * 8), y_stop, digit.into()))
            .collect();
        self.current_digits = digits;
    }

    fn update_time(&mut self, now: DateTime<Local>) {
        let digits = time_to_digits(now);
        for (i, board) in self.boards.iter_mut().enumerate() {
            if self.current_digits[i] != digits[i] {
                board.set_digit(digits[i]);
            }
        }
        self.current_digits = digits;
    }
}

fn main() -> Result<()> {
    let canvas = CrosstermCanvas::new();
    let input = CrosstermInputState::new();

    let mut state = State::new();
    state.initialize_time(canvas.width(), canvas.height());

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

            for board in s.boards.iter_mut() {
                board.update(canvas);
            }

            let now = Local::now();
            if now.signed_duration_since(s.last_update_time) > Duration::seconds(5) {
                s.update_time(now);
                s.last_update_time = now;
            }

            Ok(())
        },
        |e, s, i, canvas, dt| {
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            for board in s.boards.iter() {
                board.render(canvas);
            }

            canvas.render()?;

            Ok(())
        },
    );
}
