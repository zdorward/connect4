use yew::prelude::*;
use yew::{function_component, html};
use rand::prelude::*;
// use std::cmp::Ordering;

#[derive(Clone, PartialEq, Debug)] 
enum Cell {
    Empty,
    T,
    O,
}

enum GameState {
    Ongoing,
    WonBy(Cell),
}

#[function_component(TootBoard)]
pub fn connect_4_board() -> Html {
    let num_rows = 6; // Define the number of rows
    let num_cols = 7; // Define the number of columns

    let board = use_state(|| vec![vec![Cell::Empty; num_rows]; num_cols]); // Use variables for board size
    let player_turn = use_state(|| Cell::T); // Changed initial player to "T"
    let game_state = use_state(|| GameState::Ongoing);

    html! {
        <>
            <div class="board">
                { for (0..num_cols).map(|x| {
                    let board = board.clone();
                    let player_turn = player_turn.clone();
                    let game_state = game_state.clone();
                    let onclick = {
                        let board = board.clone();
                        let player_turn = player_turn.clone();
                        let game_state = game_state.clone();
                        Callback::from(move |_| {
                            if matches!(*game_state, GameState::Ongoing) {
                                let mut new_board = (*board).clone();
                                let column_filled = new_board[x].iter().all(|cell| matches!(cell, Cell::O) || matches!(cell, Cell::T)); // Changed check to "T"
                                if !column_filled {
                                    for y in (0..num_rows).rev() {
                                        if matches!(new_board[x][y], Cell::Empty) {
                                            new_board[x][y] = (*player_turn).clone();
                                            
                                            if check_for_win(&new_board) {
                                                game_state.set(GameState::WonBy((*player_turn).clone()));
                                                break;
                                            }
                                            
                                            player_turn.set(match *player_turn {
                                                Cell::T => Cell::O, // Changed player turn
                                                Cell::O => Cell::T, // Changed player turn
                                                _ => unreachable!(),
                                            });
                                            break;
                                        }
                                    }
                                    board.set(new_board);
                                }
                            }
                        })
                    };
                    html! {
                        <div class="column" {onclick}>
                            { for (0..num_rows).map(|y| {
                                let cell = board[x][y].clone();
                                let symbol = match cell {
                                    Cell::T => "T", // Changed symbol from "X" to "T"
                                    Cell::O => "O",
                                    Cell::Empty => "",
                                };
                                html! {
                                    <div class="cell">
                                        {symbol}
                                    </div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
            <p>
                {
                    match *game_state {
                        GameState::WonBy(Cell::T) => "Player T won!".to_string(), // Changed player symbol
                        GameState::WonBy(Cell::O) => "Player O won!".to_string(),
                        _ => "".to_string(),
                    }
                }
            </p>
        </>
    }
}


fn check_for_win(board: &Vec<Vec<Cell>>) -> bool {
    let rows = board.len();
    let cols = board[0].len();

    // Check for "TOOT" for player 1 and "OTTO" for player 2
    let win_sequences = [vec![Cell::T, Cell::O, Cell::O, Cell::T], vec![Cell::O, Cell::T, Cell::T, Cell::O]];

    // Check horizontal lines
    for y in 0..rows {
        for x in 0..cols - 3 {
            for sequence in win_sequences.iter() {
                if (0..4).all(|i| board[y][x + i] == sequence[i]) {
                    return true;
                }
            }
        }
    }

    // Check vertical lines
    for x in 0..cols {
        for y in 0..rows - 3 {
            for sequence in win_sequences.iter() {
                if (0..4).all(|i| board[y + i][x] == sequence[i]) {
                    return true;
                }
            }
        }
    }

    // Check diagonal lines
    for y in 0..rows - 3 {
        for x in 0..cols - 3 {
            for sequence in win_sequences.iter() {
                // Down-right
                if (0..4).all(|i| board[y + i][x + i] == sequence[i]) {
                    return true;
                }
                // Up-right
                if y >= 3 && (0..4).all(|i| board[y - i][x + i] == sequence[i]) {
                    return true;
                }
            }
        }
    }

    false
}