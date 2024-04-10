use yew::prelude::*;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom; // Import SliceRandom to use the choose method


use crate::components::game_types::{BoardProps, Cell, GameVersion, GameState, Difficulty}; 

fn connect4_bot(
    board: UseStateHandle<Vec<Vec<Cell>>>, 
    difficulty: UseStateHandle<Difficulty>
) {
    // let mut rng = thread_rng();
    // let cols = board.len();
    // let mut available_cols: Vec<usize> = (0..cols).filter(|&col| {
    //     board[col].iter().any(|&cell| cell == Cell::Empty)
    // }).collect();

}


fn bot_move(
    board: UseStateHandle<Vec<Vec<Cell>>>, 
    game_version: UseStateHandle<GameVersion>,
    difficulty: UseStateHandle<Difficulty>
) {

}