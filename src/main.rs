mod animation;
mod tetromino;

use animation::Digit;
use anyhow::Result;
use chrono::{DateTime, Duration, Local, NaiveTime, Timelike};
use clap::{arg, Parser};
use pixel_loop::canvas::CrosstermCanvas;
use pixel_loop::canvas::{Canvas, RenderableCanvas};
use pixel_loop::color::Color;
use pixel_loop::input::{CrosstermInputState, KeyboardKey, KeyboardState};
use tetromino::{Board, Colorscheme, DigitBoard, Rotation, Shape};

fn time_string_to_digits<T: AsRef<str>>(time_string: T) -> Vec<Digit> {
    time_string
        .as_ref()
        .chars()
        .map(|c| Digit::from(c.to_digit(10).unwrap()))
        .collect()
}

#[derive(Debug, Clone)]
enum Mode {
    Clock,
    Countdown(DateTime<Local>),
    Stopwatch(DateTime<Local>),
}

impl Default for Mode {
    fn default() -> Self {
        Self::Clock
    }
}

impl Mode {
    fn get_timestring(&self) -> String {
        match self {
            Self::Clock => Local::now().format("%H%M%S").to_string(),
            Self::Countdown(end) => {
                let duration = end.signed_duration_since(Local::now());
                // negative duration simply returns 000000
                if duration.num_seconds() < 0 {
                    return "000000".to_string();
                }
                let hours = duration.num_hours();
                let minutes = duration.num_minutes() % 60;
                let seconds = duration.num_seconds() % 60;
                format!("{:02}{:02}{:02}", hours, minutes, seconds)
            }
            Self::Stopwatch(start) => {
                let duration = Local::now().signed_duration_since(*start);
                let hours = duration.num_hours();
                let minutes = duration.num_minutes() % 60;
                let seconds = duration.num_seconds() % 60;
                format!("{:02}{:02}{:02}", hours, minutes, seconds)
            }
        }
    }
}

struct State {
    digit_boards: Vec<DigitBoard>,
    current_digits: Vec<Digit>,
    seperator_boards: Vec<Board>,
    last_update_time: DateTime<Local>,
    colorscheme: Colorscheme,
    mode: Mode,
}

impl State {
    fn new(mode: Mode, colorscheme: Colorscheme) -> Self {
        Self {
            digit_boards: vec![],
            current_digits: vec![],
            seperator_boards: vec![],
            last_update_time: Local::now(),
            mode,
            colorscheme,
        }
    }

    fn resize_canvas(&mut self, width: u32, height: u32) {
        // Each digit is 6x10
        // Spacing 2 between each
        // There are 6 digits
        // -> height: 10
        // -> width: 6*6 + 5*2
        let x_start = (width as i64 - 6 * 6 - 5 * 2) / 2;
        let y_stop = (height as i64 + 10) / 2;
        let now = Local::now();
        let digits = time_string_to_digits(self.mode.get_timestring());

        let colorscheme = self.colorscheme.clone();
        self.digit_boards = digits
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, digit)| {
                // @TODO: This is ugly as hell, but it is late and my brain
                // doesn't want to come up with something nicer here at the
                // moment ;)
                let x = x_start
                    + match i {
                        0 => 0,
                        1 => 6 + 2,
                        2 => 6 + 2 + 6 + 6,
                        3 => 6 + 2 + 6 + 6 + 6 + 2,
                        4 => 6 + 2 + 6 + 6 + 6 + 2 + 6 + 6,
                        5 => 6 + 2 + 6 + 6 + 6 + 2 + 6 + 6 + 6 + 2,
                        _ => panic!("unknown digit position {}", i),
                    };
                DigitBoard::new(i, x, y_stop, colorscheme, digit.into())
            })
            .collect();
        self.current_digits = digits;
        self.seperator_boards = vec![
            // @TODO: This is ugly as hell, but it is late and my brain
            // doesn't want to come up with something nicer here at the
            // moment ;)
            Board::new(x_start + (6 + 2 + 6 + 2), 0, y_stop - 2),
            Board::new(x_start + (6 + 2 + 6 + 2), -4, y_stop - 6),
            Board::new(x_start + (6 + 2 + 6 + 6 + 6 + 2 + 6 + 2), 0, y_stop - 2),
            Board::new(x_start + (6 + 2 + 6 + 6 + 6 + 2 + 6 + 2), -4, y_stop - 6),
        ];

        let color = self.colorscheme.apply(Shape::O, Digit::Zero, 0);
        for board in self.seperator_boards.iter_mut() {
            board.add_tetromino(0, 0, color, Shape::O, Rotation::NoRotation);
        }
    }

    fn update_time(&mut self, digits: Vec<Digit>) {
        for (i, board) in self.digit_boards.iter_mut().enumerate() {
            if self.current_digits[i] != digits[i] {
                board.set_digit(digits[i]);
            }
        }
        self.current_digits = digits;
    }
}
#[derive(Parser, Debug)]
#[command(
    author = "Jakob Westhoff <jakob@westhoffswelt.de>",
    about = "TetroTime - Time meets Tetris!"
)]
struct Args {
    #[arg(short = 'c', long, group = "mode", help = "Show a clock")]
    clock: bool,
    #[arg(short = 'w', long, group = "mode", help = "Show a stopwatch")]
    stopwatch: bool,
    #[arg(
        short = 'd',
        long,
        group = "mode",
        help = "Show a countdown (Duration in HHMMSS or HH:MM:SS)",
        value_name = "DURATION"
    )]
    countdown: Option<String>,
    #[arg(short='s', long, value_enum, default_value_t = Colorscheme::default(), help = "Select a specific colorscheme")]
    colorscheme: Colorscheme,
}

fn get_mode_from_args(args: &Args) -> Result<Mode> {
    if args.clock {
        Ok(Mode::Clock)
    } else if args.stopwatch {
        Ok(Mode::Stopwatch(Local::now()))
    } else if let Some(countdown) = &args.countdown {
        let time = NaiveTime::parse_from_str(countdown, "%H:%M:%S").unwrap_or_else(|_| {
            NaiveTime::parse_from_str(countdown, "%H%M%S")
                .unwrap_or(NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap())
        });
        Ok(Mode::Countdown(
            Local::now() + Duration::seconds(time.num_seconds_from_midnight() as i64),
        ))
    } else {
        Ok(Mode::default())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mode = get_mode_from_args(&args)?;

    let canvas = CrosstermCanvas::new();
    let input = CrosstermInputState::new();

    let mut state = State::new(mode, args.colorscheme);
    state.resize_canvas(canvas.width(), canvas.height());

    eprintln!("Render size: {}x{}", canvas.width(), canvas.height());

    pixel_loop::run(
        30,
        state,
        input,
        canvas,
        |e, s, input, canvas| {
            if let Some((width, height)) = canvas.did_resize() {
                s.resize_canvas(width, height);
            }

            if input.is_key_pressed(KeyboardKey::Q) {
                std::process::exit(0);
            }

            for board in s.digit_boards.iter_mut() {
                board.update(canvas);
            }

            for board in s.seperator_boards.iter_mut() {
                board.update(canvas);
            }

            let now = Local::now();
            if now.signed_duration_since(s.last_update_time) > Duration::seconds(5) {
                s.update_time(time_string_to_digits(s.mode.get_timestring()));
                s.last_update_time = now;
            }

            Ok(())
        },
        |e, s, i, canvas, dt| {
            canvas.clear_screen(&Color::from_rgb(0, 0, 0));

            for board in s.digit_boards.iter() {
                board.render(canvas);
            }

            for board in s.seperator_boards.iter() {
                board.render(canvas);
            }

            canvas.render()?;

            Ok(())
        },
    );
}
