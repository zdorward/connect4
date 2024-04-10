use yew::prelude::*;
use rand::{thread_rng, Rng};
use web_sys::console;


use crate::components::game_types::{BoardProps, Cell, GameVersion, GameState}; 

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let num_rows = 6;
    let num_cols = 7;

    let game_version = use_state(|| props.game_version.clone());
    let initial_turn = match *game_version {
        GameVersion::Connect4 => Cell::X,
        GameVersion::TootOtto => Cell::T,
    };
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
                { for (0..num_cols).map(|x| create_column(x, num_rows, board.clone(), player_turn.clone(), game_version.clone(), game_state.clone())) }
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
                                // player_turn.set(match (&*player_turn, &*game_version) {
                                //     (Cell::X, GameVersion::Connect4) | (Cell::T, GameVersion::TootOtto) => Cell::O,
                                //     (Cell::O, GameVersion::Connect4) => Cell::X,
                                //     (Cell::O, GameVersion::TootOtto) => Cell::T,
                                //     _ => unreachable!(),
                                // });

                                // // Update the board with the player's move
                                // board.set(new_board.clone());

                                // Make a computer move
                                if let Some(computer_board) = make_computer_move(&new_board, &player_turn, &game_state, &game_version) {
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

    let win_sequences = match game_version {
        GameVersion::Connect4 => vec![vec![Cell::X, Cell::X, Cell::X, Cell::X], vec![Cell::O, Cell::O, Cell::O, Cell::O]],
        GameVersion::TootOtto => vec![vec![Cell::T, Cell::O, Cell::O, Cell::T], vec![Cell::O, Cell::T, Cell::T, Cell::O]],
    };

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
) -> Option<Vec<Vec<Cell>>> {
    if matches!(*game_state, GameState::Ongoing) {
        let mut rng = thread_rng();
        let cols = board[0].len();
        let rows = board.len();

        // Determine the computer's cell type based on the current player's type and game version
        let computer_cell = match game_version {
            GameVersion::Connect4 => match player_turn {
                Cell::X => Cell::O,
                Cell::O => Cell::X,
                _ => unreachable!(),
            },
            GameVersion::TootOtto => {
                // In TootOtto mode, randomly choose between T and O for the computer's move
                if rng.gen_bool(0.5) { Cell::T } else { Cell::O }
            },
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
    }

    None
}
