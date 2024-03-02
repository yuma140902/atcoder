use std::{collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Color {
    Gray,
    Brown,
    Green,
    LightBlue,
    Blue,
    Yellow,
    Orange,
    Red,
    Any,
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        if value <= 399 {
            Self::Gray
        } else if value <= 799 {
            Self::Brown
        } else if value <= 1199 {
            Self::Green
        } else if value <= 1599 {
            Self::LightBlue
        } else if value <= 1999 {
            Self::Blue
        } else if value <= 2399 {
            Self::Yellow
        } else if value <= 2799 {
            Self::Orange
        } else if value <= 3199 {
            Self::Red
        } else {
            Self::Any
        }
    }
}

struct Ans(usize, usize);

impl Display for Ans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[argio::argio]
fn main(n: usize, a: [u32; n]) -> Ans {
    let colors: Vec<Color> = a.iter().map(|&i| i.into()).collect();

    let mut unique_colors = colors.iter().copied().collect::<HashSet<_>>().len();

    let num_any = colors.iter().filter(|c| **c == Color::Any).count();

    if num_any >= 1 {
        unique_colors -= 1;
    }

    let min = if unique_colors == 0 && num_any == 0 {
        0
    } else if unique_colors == 0 {
        1
    } else {
        unique_colors
    };
    let max = unique_colors + num_any;

    Ans(min, max)
}
