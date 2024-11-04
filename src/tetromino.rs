use pixel_loop::canvas::Canvas;
use pixel_loop::color::Color;
use pixel_loop::rand;

use crate::digits::{Digit, FallingTetromino};

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
    virtual_y_stop: i64,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tetrominos: vec![],
            // @FIXME: Calculate based on terminal height and shown digits
            // height, to center display.
            virtual_y_stop: 20,
        }
    }

    pub fn add_tetromino(
        &mut self,
        x: i64,
        y: i64,
        color: Color,
        shape: Shape,
        rotation: Rotation,
    ) {
        self.tetrominos.push(Tetromino {
            x,
            y,
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

            if tetromino.y == self.virtual_y_stop && tetromino.fall != FallState::Out {
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
    x_offset: i64,
    animation: Vec<FallingTetromino>,
    index: usize,
    updates_since_last_anim: usize,
}

impl DigitBoard {
    pub fn new(x_offset: i64) -> Self {
        Self {
            board: Board::new(),
            x_offset,
            animation: Digit::Zero.into(),
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

            let color = Color::from_rgb(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
            );
            self.board
                .add_tetromino(self.x_offset + dx, 0, color, shape, rotation);

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
        self.animation = digit.into();
        self.index = 0;
        self.updates_since_last_anim = 0;
    }
}
