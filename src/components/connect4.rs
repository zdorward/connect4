use yew::prelude::*;
use rand::{thread_rng, Rng};
use web_sys::console;


use crate::components::game_types::{BoardProps, Cell, GameVersion, GameState, Difficulty};

use super::game_types::_BoardProps::difficulty; 

#[function_component(Connect4Board)]
pub fn board(props: &BoardProps) -> Html {
    let num_rows = 6;
    let num_cols = 7;

    let game_version = use_state(|| props.game_version.clone());
    let game_difficulty = use_state(|| props.difficulty.clone());
    let initial_turn = Cell::X;
    let board = use_state(|| vec![vec![Cell::Empty; num_rows]; num_cols]);
    let player_turn = use_state(move || initial_turn);
    let game_state = use_state(|| GameState::Ongoing);

    html! {
        <>
            <h1>{ format!("Welcome to {}", match *game_version {
                GameVersion::Connect4 => "Connect4",
                GameVersion::TootOtto => "Toot and Otto",
            }) }</h1>
            <div class="board">
                { for (0..num_cols).map(|x| create_column(x, num_rows, board.clone(), player_turn.clone(), game_version.clone(), game_state.clone(), game_difficulty.clone())) }
            </div>
            <p>
                {
                    match *game_state {
                        GameState::WonBy(Cell::T) | GameState::WonBy(Cell::X) => "Player 1 won!".to_string(),
                        GameState::WonBy(Cell::O) => "Player 2 won!".to_string(),
                        _ => "".to_string(),
                    }
                }
            </p>
        </>
    }
}

fn create_column(
    x: usize,
    num_rows: usize,
    board: UseStateHandle<Vec<Vec<Cell>>>,
    player_turn: UseStateHandle<Cell>,
    game_version: UseStateHandle<GameVersion>,
    game_state: UseStateHandle<GameState>,
    game_difficulty: UseStateHandle<Difficulty>
) -> Html {
    let onclick = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let game_version = game_version.clone();
        let game_state = game_state.clone();
        Callback::from(move |_| {
            if matches!(*game_state, GameState::Ongoing) {
                let mut new_board = (*board).clone();
                let column_filled = new_board[x].iter().all(|cell| *cell != Cell::Empty);
                if !column_filled {
                    for y in (0..num_rows).rev() {
                        if matches!(new_board[x][y], Cell::Empty) {
                            new_board[x][y] = (*player_turn).clone();
                            
                            if check_for_win(&new_board, &game_version) {
                                board.set(new_board.clone());
                                game_state.set(GameState::WonBy((*player_turn).clone()));
                            } else {
                                if let Some(computer_board) = make_computer_move(&new_board, &player_turn, &game_state, &game_version, &game_difficulty) {
                                    // Check if the computer's move resulted in a win.
                                    if check_for_win(&computer_board, &game_version) {
                                        game_state.set(GameState::WonBy(match *player_turn {
                                            // Assuming the computer plays the opposite of the player's turn
                                            Cell::X => Cell::O,
                                            Cell::O => Cell::X,
                                            _ => unreachable!(),
                                        }));
                                    }
                                    board.set(computer_board);
                                }
                            }
                            break;
                        }
                    }
                }
            }
        })
    };
    html! {
        <div class="column" {onclick}>
            { for (0..num_rows).map(|y| {
                let cell = board[x][y].clone();
                let symbol = match cell {
                    Cell::X => "X",
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

fn check_for_win(board: &Vec<Vec<Cell>>, game_version: &GameVersion) -> bool {

    let rows = board.len();
    let cols = board[0].len();

    let win_sequences = vec![vec![Cell::X, Cell::X, Cell::X, Cell::X], vec![Cell::O, Cell::O, Cell::O, Cell::O]];

    // Check horizontal lines
    for y in 0..rows {
        for x in 0..cols - 3 {
            for sequence in &win_sequences {
                if (0..4).all(|i| board[y][x + i] == sequence[i]) {
                    return true;
                }
            }
        }
    }

    // Check vertical lines
    for x in 0..cols {
        for y in 0..rows - 3 {
            for sequence in &win_sequences {
                if (0..4).all(|i| board[y + i][x] == sequence[i]) {
                    return true;
                }
            }
        }
    }

    // Check diagonal lines
    for y in 0..rows - 3 {
        for x in 0..cols - 3 {
            for sequence in &win_sequences {
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

fn make_computer_move(
    board: &Vec<Vec<Cell>>,
    player_turn: &Cell,
    game_state: &GameState,
    game_version: &GameVersion,
    game_difficulty: &Difficulty
) -> Option<Vec<Vec<Cell>>> {
    if matches!(*game_state, GameState::Ongoing) {

        let mut rng = thread_rng();
        let cols = board[0].len();
        let rows = board.len();

        match game_difficulty {
            Difficulty::Easy => {
                // Determine the computer's cell type based on the current player's type and game version
                let computer_cell =  match player_turn {
                        Cell::X => Cell::O,
                        Cell::O => Cell::X,
                        _ => unreachable!(),
                    };


                // Attempt to place the computer's piece in a random column
                for _ in 0..cols {
                    let col = rng.gen_range(0..cols);
                    for row in (0..rows - 1).rev() {
                        if matches!(board[col][row], Cell::Empty) {
                            let mut new_board = board.clone();
                            new_board[col][row] = computer_cell;
                            return Some(new_board);
                        }
                    }
                }
            },
            Difficulty::Hard => {

            },
        }
    }

    None
}