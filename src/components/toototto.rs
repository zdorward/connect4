use yew::prelude::*;
// use web_sys::console;
use rand::{thread_rng, Rng};
use crate::components::game_types::Difficulty;

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

#[derive(Clone, PartialEq, Properties)]
pub struct BoardProps {
    pub difficulty: Difficulty
}

#[function_component(TootOttoBoard)]
pub fn connect_4_board(props: &BoardProps) -> Html {
    let num_rows = 6;
    let num_cols = 7;

    let board = use_state(|| vec![vec![Cell::Empty; num_rows]; num_cols]);
    let player_turn = use_state(|| Cell::T); // Assuming player starts with 'T'
    let game_state = use_state(|| GameState::Ongoing);
    let game_difficulty = use_state(|| props.difficulty.clone());

    html! {
        <>
            <h1>{ format!("Welcome to Toot and Otto")}</h1>
            <div class="board">
                { for (0..num_cols).map(|x| create_column(x, num_rows, board.clone(), player_turn.clone(), game_state.clone(), game_difficulty.clone())) }
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

fn create_column(
    x: usize,
    num_rows: usize,
    board: UseStateHandle<Vec<Vec<Cell>>>,
    player_turn: UseStateHandle<Cell>,
    game_state: UseStateHandle<GameState>,
    game_difficulty: UseStateHandle<Difficulty>
) -> Html {
    let onclick = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let game_state = game_state.clone();
        let game_difficulty = game_difficulty.clone();
        Callback::from(move |_| {
            if matches!(*game_state, GameState::Ongoing) {
                let mut new_board = (*board).clone();
                if let Some(updated_board) = make_player_move(x, num_rows, &new_board, &player_turn) {
                    new_board = updated_board;
                    let win_state = check_for_win(&new_board);
                    if win_state != Cell::Empty {
                        game_state.set(GameState::WonBy(win_state));
                    } else {
                        // Computer's turn to play after player's move
                        if let Some(computer_board) = make_computer_move(&new_board, &game_state, &game_difficulty) {
                            new_board = computer_board;
                            let computer_win_state = check_for_win(&new_board);
                            if computer_win_state != Cell::Empty {
                                game_state.set(GameState::WonBy(computer_win_state));
                            }
                        }
                    }
                    board.set(new_board);
                }
            }
        })
    };

    // Render the column with cells
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

fn make_player_move(
    x: usize,
    num_rows: usize,
    board: &Vec<Vec<Cell>>,
    player_turn: &Cell,
) -> Option<Vec<Vec<Cell>>> {
    let mut new_board = board.clone();
    let column_filled = new_board[x].iter().all(|cell| !matches!(cell, Cell::Empty));
    if !column_filled {
        for y in (0..num_rows).rev() {
            if matches!(new_board[x][y], Cell::Empty) {
                new_board[x][y] = player_turn.clone();
                return Some(new_board);
            }
        }
    }
    None
}

fn make_computer_move(
    board: &Vec<Vec<Cell>>,
    game_state: &GameState,
    game_difficulty: &Difficulty
) -> Option<Vec<Vec<Cell>>> {
    if matches!(*game_state, GameState::Ongoing) {

        let mut rng = thread_rng();
        let cols = board[0].len();
        let rows = board.len();

        match game_difficulty {
            Difficulty::Easy => {
                // Determine the computer's cell type based on the current player's type and game version
                let computer_cell = if rng.gen_bool(0.5) { Cell::T } else { Cell::O };

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

fn check_for_win(board: &Vec<Vec<Cell>>) -> Cell {
    let rows = board.len();
    let cols = board[0].len();

    // Define the winning sequences for each player.
    let toot_sequence = vec![Cell::T, Cell::O, Cell::O, Cell::T];
    let otto_sequence = vec![Cell::O, Cell::T, Cell::T, Cell::O];

    // Check horizontal lines for a win.
    for y in 0..rows {
        for x in 0..cols - 3 {
            if (0..4).all(|i| board[y][x + i] == toot_sequence[i]) {
                return Cell::T;
            }
            if (0..4).all(|i| board[y][x + i] == otto_sequence[i]) {
                return Cell::O;
            }
        }
    }

    // Check vertical lines for a win.
    for x in 0..cols {
        for y in 0..rows - 3 {
            if (0..4).all(|i| board[y + i][x] == toot_sequence[i]) {
                return Cell::T;
            }
            if (0..4).all(|i| board[y + i][x] == otto_sequence[i]) {
                return Cell::O;
            }
        }
    }

    // Check diagonals for a win (down to the right).
    for y in 0..rows - 3 {
        for x in 0..cols - 3 {
            if (0..4).all(|i| board[y + i][x + i] == toot_sequence[i]) {
                return Cell::T;
            }
            if (0..4).all(|i| board[y + i][x + i] == otto_sequence[i]) {
                return Cell::O;
            }
        }
    }

    // Check diagonals for a win (up to the right).
    for y in 3..rows {
        for x in 0..cols - 3 {
            if (0..4).all(|i| board[y - i][x + i] == toot_sequence[i]) {
                return Cell::T;
            }
            if (0..4).all(|i| board[y - i][x + i] == otto_sequence[i]) {
                return Cell::O;
            }
        }
    }

    // If no win condition is met, return Cell::Empty.
    Cell::Empty
}
