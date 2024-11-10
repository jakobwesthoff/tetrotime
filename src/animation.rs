use crate::tetromino::{Rotation, Shape};

#[derive(Clone, Copy, PartialEq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<usize> for Digit {
    fn from(value: usize) -> Self {
        match value {
            0 => Digit::Zero,
            1 => Digit::One,
            2 => Digit::Two,
            3 => Digit::Three,
            4 => Digit::Four,
            5 => Digit::Five,
            6 => Digit::Six,
            7 => Digit::Seven,
            8 => Digit::Eight,
            9 => Digit::Nine,
            _ => panic!(
                "Conversion into Digit beyond 0-9 not supported. Got {}",
                value
            ),
        }
    }
}

impl From<u32> for Digit {
    fn from(value: u32) -> Self {
        Self::from(value as usize)
    }
}

pub type Animation = Vec<FallingTetromino>;

#[derive(Debug, Copy, Clone)]
pub struct FallingTetromino {
    pub shape: Shape,
    pub rotation: Rotation,
    pub dx: i64,
}

impl FallingTetromino {
    pub fn new(shape: Shape, rotation: Rotation, dx: i64) -> Self {
        Self {
            shape,
            rotation,
            dx,
        }
    }
}

impl Into<Animation> for Digit {
    fn into(self) -> Animation {
        use Rotation::*;
        use Shape::*;
        match self {
            Digit::Zero => vec![
                FallingTetromino::new(J, Degrees90, 4),
                FallingTetromino::new(I, Degrees90, 0),
                FallingTetromino::new(S, NoRotation, 2),
                FallingTetromino::new(T, Degrees270, 1),
                FallingTetromino::new(T, Degrees90, 1),
                FallingTetromino::new(Z, Degrees90, 4),
                FallingTetromino::new(Z, Degrees90, 4),
                FallingTetromino::new(T, Degrees270, 4),
                FallingTetromino::new(Z, Degrees90, 0),
                FallingTetromino::new(T, Degrees270, 0),
                FallingTetromino::new(Z, NoRotation, 2),
                FallingTetromino::new(J, NoRotation, 5),
            ],
            Digit::One => vec![
                FallingTetromino::new(O, NoRotation, 4),
                FallingTetromino::new(L, Degrees270, 4),
                FallingTetromino::new(I, Degrees90, 5),
                FallingTetromino::new(J, Degrees270, 4),
                FallingTetromino::new(O, NoRotation, 4),
            ],
            Digit::Two => vec![
                FallingTetromino::new(I, Degrees90, 0),
                FallingTetromino::new(O, NoRotation, 4),
                FallingTetromino::new(L, Degrees180, 1),
                FallingTetromino::new(L, Degrees270, 1),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(J, NoRotation, 5),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(O, NoRotation, 4),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(J, NoRotation, 5),
                FallingTetromino::new(L, NoRotation, 0),
            ],
            Digit::Three => vec![
                FallingTetromino::new(I, NoRotation, 2),
                FallingTetromino::new(O, NoRotation, 0),
                FallingTetromino::new(L, Degrees180, 2),
                FallingTetromino::new(L, Degrees90, 5),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(J, NoRotation, 5),
                FallingTetromino::new(O, NoRotation, 4),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(J, NoRotation, 5),
            ],
            Digit::Four => vec![
                FallingTetromino::new(J, Degrees90, 4),
                FallingTetromino::new(J, Degrees270, 4),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(O, NoRotation, 0),
                FallingTetromino::new(O, NoRotation, 0),
                FallingTetromino::new(J, NoRotation, 5),
                FallingTetromino::new(J, Degrees90, 4),
                FallingTetromino::new(J, Degrees270, 4),
            ],
            Digit::Five => vec![
                FallingTetromino::new(J, Degrees90, 4),
                FallingTetromino::new(J, Degrees270, 4),
                FallingTetromino::new(I, NoRotation, 0),
                FallingTetromino::new(I, NoRotation, 0),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(J, NoRotation, 5),
                FallingTetromino::new(O, NoRotation, 0),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(J, NoRotation, 5),
                FallingTetromino::new(L, NoRotation, 0),
            ],
            Digit::Six => vec![
                FallingTetromino::new(J, Degrees90, 4),
                FallingTetromino::new(J, Degrees270, 4),
                FallingTetromino::new(I, NoRotation, 0),
                FallingTetromino::new(I, NoRotation, 0),
                FallingTetromino::new(O, NoRotation, 0),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(J, NoRotation, 5),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(O, NoRotation, 0),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(J, NoRotation, 5),
            ],
            Digit::Seven => vec![
                FallingTetromino::new(O, NoRotation, 4),
                FallingTetromino::new(L, Degrees270, 4),
                FallingTetromino::new(I, Degrees90, 5),
                FallingTetromino::new(J, Degrees270, 4),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(L, NoRotation, 0),
                FallingTetromino::new(J, NoRotation, 5),
            ],
            Digit::Eight => vec![
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(L, Degrees180, 2),
                FallingTetromino::new(T, Degrees270, 0),
                FallingTetromino::new(I, Degrees90, 5),
                FallingTetromino::new(S, Degrees90, 1),
                FallingTetromino::new(L, Degrees180, 1),
                FallingTetromino::new(T, Degrees270, 4),
                FallingTetromino::new(S, Degrees90, 5),
                FallingTetromino::new(J, Degrees180, 0),
                FallingTetromino::new(S, Degrees90, 1),
                FallingTetromino::new(Z, NoRotation, 1),
                FallingTetromino::new(Z, NoRotation, 3),
                FallingTetromino::new(L, Degrees90, 5),
            ],
            Digit::Nine => vec![
                FallingTetromino::new(O, NoRotation, 0),
                FallingTetromino::new(L, Degrees180, 3),
                FallingTetromino::new(L, NoRotation, 2),
                FallingTetromino::new(O, NoRotation, 4),
                FallingTetromino::new(I, Degrees90, 5),
                FallingTetromino::new(I, NoRotation, 1),
                FallingTetromino::new(T, Degrees270, 0),
                FallingTetromino::new(L, Degrees180, 2),
                FallingTetromino::new(Z, Degrees90, 4),
                FallingTetromino::new(J, Degrees270, 1),
                FallingTetromino::new(T, NoRotation, 3),
                FallingTetromino::new(J, Degrees270, 0),
            ],
        }
    }
}
