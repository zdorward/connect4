use yew::prelude::*;
use yew::{function_component, html};
use rand::prelude::*;
// use std::cmp::Ordering;

#[derive(Clone, PartialEq)] 
enum Cell {
    Empty,
    T,
    O,
}

#[derive(Clone, PartialEq)] 
enum GameState {
    Ongoing,
    WonBy(Cell),
}

fn create_column(
    x: usize,
    num_rows: usize,
    board: UseStateHandle<Vec<Vec<Cell>>>,
    player_turn: UseStateHandle<Cell>,
    game_state: UseStateHandle<GameState>,
) -> Html {
    let onclick = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let game_state = game_state.clone();
        Callback::from(move |_| {
            if matches!(*game_state, GameState::Ongoing) {
                let mut new_board = (*board).clone();
                let column_filled = new_board[x].iter().all(|cell| matches!(cell, Cell::O) || matches!(cell, Cell::T));
                if !column_filled {
                    for y in (0..num_rows).rev() {
                        if matches!(new_board[x][y], Cell::Empty) {
                            new_board[x][y] = (*player_turn).clone();
                            
                            if check_for_win(&new_board) {
                                game_state.set(GameState::WonBy((*player_turn).clone()));
                                break;
                            }
                            
                            player_turn.set(match *player_turn {
                                Cell::T => Cell::O,
                                Cell::O => Cell::T,
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
                    Cell::T => "T",
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

#[function_component(TootBoard)]
pub fn connect_4_board() -> Html {
    let num_rows = 6;
    let num_cols = 7;

    let board = use_state(|| vec![vec![Cell::Empty; num_rows]; num_cols]);
    let player_turn = use_state(|| Cell::T);
    let game_state = use_state(|| GameState::Ongoing);

    html! {
        <>
            <div class="board">
                { for (0..num_cols).map(|x| create_column(x, num_rows, board.clone(), player_turn.clone(), game_state.clone())) }
            </div>
            <p>
                {
                    match *game_state {
                        GameState::WonBy(Cell::T) => "Player T won!".to_string(),
                        GameState::WonBy(Cell::O) => "Player O won!".to_string(),
                        _ => "".to_string(),
                    }
                }
            </p>
        </>
    }
}