use clap::ValueEnum;
use pixel_loop::canvas::Canvas;
use pixel_loop::color::Color;

use crate::animation::{Animation, Digit, FallingTetromino};

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
#[clap(rename_all = "lowercase")]
pub enum Colorscheme {
    #[value(help = "Classic Tetris colors with bright, distinct shades")]
    Original,
    #[value(help = "Different shades of gray for a monochrome look")]
    Grayscale,
    #[value(help = "Colors based on the digit position in the time display")]
    Position,
    #[value(help = "Each digit (0-9) gets its own distinct color")]
    Digit,
    #[value(help = "Vibrant neon colors inspired by cyberpunk aesthetics")]
    Neon,
    #[value(help = "Soft, muted pastel colors for a gentle appearance")]
    Pastel,
    #[value(help = "Shades of blue and turquoise inspired by ocean depths")]
    Ocean,
    #[value(help = "Warm fall colors with reds, browns and golden tones")]
    Autumn,
    #[value(help = "Traditional Christmas theme with reds, greens and gold")]
    Christmas,
    #[value(help = "Warm spectrum focusing on yellows and oranges")]
    Warm,
    #[value(help = "Digital green shades inspired by The Matrix movie")]
    Matrix,
    #[value(help = "Rich and deep purple/violet color palette")]
    Purple,
}

impl Default for Colorscheme {
    fn default() -> Self {
        Self::Original
    }
}

impl Colorscheme {
    pub fn apply(&self, shape: Shape, digit: Digit, position: usize) -> Color {
        match self {
            Colorscheme::Original => match shape {
                Shape::L => Color::from_rgb(223, 155, 42),
                Shape::J => Color::from_rgb(22, 0, 230),
                Shape::O => Color::from_rgb(237, 238, 57),
                Shape::T => Color::from_rgb(136, 26, 231),
                Shape::I => Color::from_rgb(103, 232, 236),
                Shape::S => Color::from_rgb(100, 233, 49),
                Shape::Z => Color::from_rgb(213, 50, 27),
            },
            Colorscheme::Grayscale => match shape {
                Shape::L => Color::from_rgb(178, 178, 178),
                Shape::J => Color::from_rgb(74, 74, 74),
                Shape::O => Color::from_rgb(234, 234, 234),
                Shape::T => Color::from_rgb(98, 98, 98),
                Shape::I => Color::from_rgb(216, 216, 216),
                Shape::S => Color::from_rgb(200, 200, 200),
                Shape::Z => Color::from_rgb(120, 120, 120),
            },
            Colorscheme::Position => match position {
                0 => Color::from_rgb(237, 238, 57),
                1 => Color::from_rgb(213, 50, 27),
                2 => Color::from_rgb(223, 155, 42),
                3 => Color::from_rgb(136, 26, 231),
                4 => Color::from_rgb(103, 232, 236),
                5 => Color::from_rgb(100, 233, 49),
                6 => Color::from_rgb(213, 50, 27),
                _ => panic!("Unknown position for Colorscheme 'position' {}", position),
            },
            Colorscheme::Digit => match digit {
                Digit::Zero => Color::from_rgb(180, 180, 180),
                Digit::One => Color::from_rgb(255, 215, 0),
                Digit::Two => Color::from_rgb(255, 105, 180),
                Digit::Three => Color::from_rgb(0, 139, 139),
                Digit::Four => Color::from_rgb(255, 160, 122),
                Digit::Five => Color::from_rgb(147, 112, 219),
                Digit::Six => Color::from_rgb(32, 178, 170),
                Digit::Seven => Color::from_rgb(255, 218, 185),
                Digit::Eight => Color::from_rgb(176, 196, 222),
                Digit::Nine => Color::from_rgb(255, 192, 203),
            },
            Colorscheme::Neon => match shape {
                // Bright, glowing neon colors
                Shape::L => Color::from_rgb(255, 41, 117), // Hot pink
                Shape::J => Color::from_rgb(0, 255, 255),  // Cyan
                Shape::O => Color::from_rgb(255, 236, 39), // Neon yellow
                Shape::T => Color::from_rgb(167, 0, 255),  // Bright purple
                Shape::I => Color::from_rgb(0, 255, 169),  // Bright mint
                Shape::S => Color::from_rgb(123, 255, 0),  // Lime green
                Shape::Z => Color::from_rgb(255, 82, 0),   // Neon orange
            },
            Colorscheme::Pastel => match shape {
                // Soft, muted pastel colors
                Shape::L => Color::from_rgb(255, 183, 197), // Soft pink
                Shape::J => Color::from_rgb(174, 198, 255), // Pastel blue
                Shape::O => Color::from_rgb(255, 251, 150), // Pastel yellow
                Shape::T => Color::from_rgb(211, 178, 255), // Pastel purple
                Shape::I => Color::from_rgb(178, 255, 241), // Pastel turquoise
                Shape::S => Color::from_rgb(178, 255, 178), // Pastel green
                Shape::Z => Color::from_rgb(255, 178, 178), // Pastel red
            },
            Colorscheme::Ocean => match shape {
                // Colors from shallow to deep ocean
                Shape::L => Color::from_rgb(144, 224, 239), // Light aqua
                Shape::J => Color::from_rgb(0, 119, 182),   // Deep blue
                Shape::O => Color::from_rgb(202, 240, 248), // Sea foam
                Shape::T => Color::from_rgb(3, 4, 94),      // Deep ocean
                Shape::I => Color::from_rgb(0, 150, 199),   // Azure
                Shape::S => Color::from_rgb(72, 202, 228),  // Sky blue
                Shape::Z => Color::from_rgb(0, 180, 216),   // Turquoise
            },
            Colorscheme::Autumn => match shape {
                // Warm fall colors
                Shape::L => Color::from_rgb(230, 125, 35), // Burnt orange
                Shape::J => Color::from_rgb(153, 62, 29),  // Auburn
                Shape::O => Color::from_rgb(255, 190, 15), // Golden yellow
                Shape::T => Color::from_rgb(130, 48, 56),  // Wine red
                Shape::I => Color::from_rgb(217, 80, 48),  // Rust
                Shape::S => Color::from_rgb(85, 107, 47),  // Olive green
                Shape::Z => Color::from_rgb(168, 50, 27),  // Maroon
            },
            Colorscheme::Christmas => match shape {
                // Traditional Christmas colors with rich reds, greens, and gold accents
                Shape::L => Color::from_rgb(176, 0, 0), // Deep Christmas red
                Shape::J => Color::from_rgb(0, 105, 0), // Forest green
                Shape::O => Color::from_rgb(255, 215, 0), // Golden yellow
                Shape::T => Color::from_rgb(146, 22, 34), // Cardinal red
                Shape::I => Color::from_rgb(0, 140, 0), // Christmas tree green
                Shape::S => Color::from_rgb(212, 175, 55), // Metallic gold
                Shape::Z => Color::from_rgb(190, 0, 0), // Bright Christmas red
            },
            Colorscheme::Warm => match shape {
                // Warm colors focusing on yellows and oranges
                Shape::L => Color::from_rgb(255, 167, 0), // Pure orange
                Shape::J => Color::from_rgb(255, 140, 0), // Dark orange
                Shape::O => Color::from_rgb(255, 215, 0), // Golden yellow
                Shape::T => Color::from_rgb(255, 198, 0), // Amber
                Shape::I => Color::from_rgb(255, 179, 25), // Marigold
                Shape::S => Color::from_rgb(255, 126, 0), // Safety orange
                Shape::Z => Color::from_rgb(255, 103, 0), // Burnt orange
            },
            Colorscheme::Matrix => match shape {
                // Different intensities of the iconic Matrix green
                Shape::L => Color::from_rgb(0, 255, 0), // Pure Matrix green
                Shape::J => Color::from_rgb(0, 185, 0), // Medium bright green
                Shape::O => Color::from_rgb(172, 255, 172), // Light digital green
                Shape::T => Color::from_rgb(0, 140, 0), // Dark digital green
                Shape::I => Color::from_rgb(0, 215, 0), // Bright digital green
                Shape::S => Color::from_rgb(128, 255, 128), // Pale Matrix green
                Shape::Z => Color::from_rgb(0, 155, 0), // Muted Matrix green
            },
            Colorscheme::Purple => match shape {
                // Rich, deep purples and violet shades
                Shape::L => Color::from_rgb(148, 0, 211), // Dark violet
                Shape::J => Color::from_rgb(106, 13, 173), // Deep purple
                Shape::O => Color::from_rgb(178, 102, 255), // Bright violet
                Shape::T => Color::from_rgb(75, 0, 130),  // Indigo
                Shape::I => Color::from_rgb(128, 0, 128), // Pure purple
                Shape::S => Color::from_rgb(153, 50, 204), // Dark orchid
                Shape::Z => Color::from_rgb(139, 0, 139), // Deep magenta
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    L,
    J,
    O,
    T,
    I,
    S,
    Z,
}

#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    Degrees90,
    Degrees180,
    Degrees270,
    NoRotation,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum FallState {
    In,
    Out,
    Hold,
}

// Tetromino coordinates always describe the lower left corner of the shape,
// where it is filled.
// Exanmple:
// xxx
//  x
//  ^ Lower(!) left corner of this pixel is the coordinate of the tetromino.
//
// This is in contrast to the usual coordinate system where the upper left
// corner is used. Positioning that way, makes the resoning about laying
// out the tetrominos to form a clock easier in the end.
//
// This kind of "messes" up rotation, as there is no fixed "center" to rotate
// around. However as we are not in the business of implementing a tetris game
// this is not important to us. Rotationonal symetry is not a requirement for
// the clock.  The shapes are based upon this reference:
// https://tetris.wiki/images/b/b5/Tgm_basic_ars_description.png
struct Tetromino {
    shape: Shape,
    rotation: Rotation,
    x: i64,
    y: i64,
    color: Color,
    fall: FallState,
}

fn would_tetromino_collide_with_canvas<C: Canvas>(
    Tetromino {
        shape,
        rotation,
        x,
        y,
        ..
    }: &Tetromino,
    canvas: &C,
) -> bool {
    let empty = Color::from_rgb(0, 0, 0);
    use Rotation::*;
    use Shape::*;
    match (shape, rotation) {
        (L, NoRotation) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 1, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y - 1, &empty)
        }
        (L, Degrees90) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 2, &empty)
        }
        (L, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y, &empty)
        }
        (L, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
        }
        (J, NoRotation) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
                || !canvas.is_empty_or_color(*x - 2, *y - 1, &empty)
        }
        (J, Degrees90) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
        }
        (J, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y, &empty)
        }
        (J, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 2, &empty)
        }
        (O, _) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
        }
        (T, NoRotation) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 1, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
        }
        (T, Degrees90) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
        }
        (T, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y, &empty)
        }
        (T, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 1, &empty)
        }
        (I, NoRotation) | (I, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y, &empty)
                || !canvas.is_empty_or_color(*x + 3, *y, &empty)
        }
        (I, Degrees90) | (I, Degrees270) => !canvas.is_empty_or_color(*x, *y, &empty),
        (S, NoRotation) | (S, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x + 2, *y - 1, &empty)
        }
        (S, Degrees90) | (S, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
        }
        (Z, NoRotation) | (Z, Degrees180) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y, &empty)
                || !canvas.is_empty_or_color(*x - 1, *y - 1, &empty)
        }
        (Z, Degrees90) | (Z, Degrees270) => {
            !canvas.is_empty_or_color(*x, *y, &empty)
                || !canvas.is_empty_or_color(*x + 1, *y - 1, &empty)
        }
    }
}

pub struct Board {
    tetrominos: Vec<Tetromino>,
    x_offset: i64,
    y_offset: i64,
    y_stop: i64,
}

impl Board {
    pub fn new(x_offset: i64, y_offset: i64, y_stop: i64) -> Self {
        Self {
            tetrominos: vec![],
            x_offset,
            y_offset,
            y_stop,
        }
    }

    pub fn add_tetromino(
        &mut self,
        dx: i64,
        dy: i64,
        color: Color,
        shape: Shape,
        rotation: Rotation,
    ) {
        self.tetrominos.push(Tetromino {
            x: self.x_offset + dx,
            y: self.y_offset + dy,
            color,
            shape,
            rotation,
            fall: FallState::In,
        })
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) {
        for Tetromino {
            shape,
            rotation,
            x,
            y,
            color,
            ..
        } in self.tetrominos.iter()
        {
            use Rotation::*;
            use Shape::*;
            match (shape, rotation) {
                (L, NoRotation) => {
                    canvas.filled_rect(*x, *y - 2, 1, 2, color);
                    canvas.filled_rect(*x + 1, *y - 2, 2, 1, color);
                }
                (L, Degrees90) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x - 1, *y - 3, 1, 1, color);
                }
                (L, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 3, 1, color);
                    canvas.filled_rect(*x + 2, *y - 2, 1, 1, color);
                }
                (L, Degrees270) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y - 1, 1, 1, color);
                }
                (J, NoRotation) => {
                    canvas.filled_rect(*x - 2, *y - 2, 2, 1, color);
                    canvas.filled_rect(*x, *y - 2, 1, 2, color);
                }
                (J, Degrees90) => {
                    canvas.filled_rect(*x, *y - 1, 2, 1, color);
                    canvas.filled_rect(*x + 1, *y - 3, 1, 2, color);
                }
                (J, Degrees180) => {
                    canvas.filled_rect(*x, *y - 2, 1, 2, color);
                    canvas.filled_rect(*x + 1, *y - 1, 2, 1, color);
                }
                (J, Degrees270) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y - 3, 1, 1, color);
                }
                (O, _) => {
                    canvas.filled_rect(*x, *y - 2, 2, 2, color);
                }
                (T, NoRotation) => {
                    canvas.filled_rect(*x - 1, *y - 2, 3, 1, color);
                    canvas.filled_rect(*x, *y - 1, 1, 1, color);
                }
                (T, Degrees90) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x - 1, *y - 2, 1, 1, color);
                }
                (T, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 3, 1, color);
                    canvas.filled_rect(*x + 1, *y - 2, 1, 1, color);
                }
                (T, Degrees270) => {
                    canvas.filled_rect(*x, *y - 3, 1, 3, color);
                    canvas.filled_rect(*x + 1, *y - 2, 1, 1, color);
                }
                (I, NoRotation) | (I, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 4, 1, color);
                }
                (I, Degrees90) | (I, Degrees270) => {
                    canvas.filled_rect(*x, *y - 4, 1, 4, color);
                }
                (S, NoRotation) | (S, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 2, 1, color);
                    canvas.filled_rect(*x + 1, *y - 2, 2, 1, color);
                }
                (S, Degrees90) | (S, Degrees270) => {
                    canvas.filled_rect(*x, *y - 2, 1, 2, color);
                    canvas.filled_rect(*x - 1, *y - 3, 1, 2, color);
                }
                (Z, NoRotation) | (Z, Degrees180) => {
                    canvas.filled_rect(*x, *y - 1, 2, 1, color);
                    canvas.filled_rect(*x - 1, *y - 2, 2, 1, color);
                }
                (Z, Degrees90) | (Z, Degrees270) => {
                    canvas.filled_rect(*x, *y - 2, 1, 2, color);
                    canvas.filled_rect(*x + 1, *y - 3, 1, 2, color);
                }
            }
        }
    }

    pub fn update<C: Canvas>(&mut self, canvas: &C) {
        for tetromino in self.tetrominos.iter_mut() {
            if tetromino.fall != FallState::Hold
                && !would_tetromino_collide_with_canvas(tetromino, canvas)
            {
                tetromino.y += 1;
            }

            if tetromino.y == self.y_stop && tetromino.fall != FallState::Out {
                tetromino.fall = FallState::Hold;
            }
        }

        self.tetrominos
            .retain(|tetromino| tetromino.y <= canvas.height() as i64 + 4);
    }

    pub fn initiate_fall_out(&mut self) {
        for tetromino in self.tetrominos.iter_mut() {
            tetromino.fall = FallState::Out;
        }
    }
}

pub struct DigitBoard {
    board: Board,
    digit: Digit,
    animation: Animation,
    position: usize,
    colorscheme: Colorscheme,
    index: usize,
    updates_since_last_anim: usize,
}

impl DigitBoard {
    pub fn new(
        position: usize,
        x_offset: i64,
        y_stop: i64,
        colorscheme: Colorscheme,
        digit: Digit,
    ) -> Self {
        Self {
            board: Board::new(x_offset, 0, y_stop),
            position,
            digit,
            animation: digit.into(),
            colorscheme,
            index: 0,
            updates_since_last_anim: 0,
        }
    }

    pub fn update<C: Canvas>(&mut self, canvas: &C) {
        if self.index < self.animation.len() && self.updates_since_last_anim > 3 {
            let FallingTetromino {
                shape,
                rotation,
                dx,
            } = self.animation[self.index];

            let color = self.colorscheme.apply(shape, self.digit, self.position);
            self.board.add_tetromino(dx, 0, color, shape, rotation);

            self.index += 1;
            self.updates_since_last_anim = 0;
        }

        self.board.update(canvas);
        self.updates_since_last_anim += 1;
    }

    pub fn render<C: Canvas>(&self, canvas: &mut C) {
        self.board.render(canvas);
    }

    pub fn set_digit(&mut self, digit: Digit) {
        self.board.initiate_fall_out();
        self.digit = digit;
        self.animation = digit.into();
        self.index = 0;
        self.updates_since_last_anim = 0;
    }
}
