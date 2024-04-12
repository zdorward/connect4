use yew::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    pub difficulty: Difficulty,
    pub num_rows: usize,
    pub num_cols: usize,
}
#[derive(Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Hard
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ColorBlindMode {
    On,
    Off
}

impl fmt::Display for ColorBlindMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorBlindMode::On => write!(f, "On"),
            ColorBlindMode::Off => write!(f, "Off"),
        }
    }
}