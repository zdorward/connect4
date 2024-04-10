use yew::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    X,
    T,
    O,
}

#[derive(Clone, PartialEq)]
pub enum GameVersion {
    Connect4,
    TootOtto,
}

impl fmt::Display for GameVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameVersion::Connect4 => write!(f, "Connect4"),
            GameVersion::TootOtto => write!(f, "TootOtto"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum GameState {
    Ongoing,
    WonBy(Cell),
}

#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    pub game_version: GameVersion,
}