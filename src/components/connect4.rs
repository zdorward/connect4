use yew::prelude::*;
use rand::{thread_rng, Rng};

use crate::components::lib::{BoardProps, Difficulty, ColorBlindMode};

#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

#[derive(Clone, PartialEq)]
pub enum GameState {
    Ongoing,
    WonBy(Cell),
    Draw,
}


#[function_component(Connect4Board)]
pub fn board(props: &BoardProps) -> Html {

    let color_blind_mode = use_state(|| ColorBlindMode::Off);
    let game_difficulty = use_state(|| props.difficulty.clone());
    let initial_turn = Cell::X;
    let board = use_state(|| vec![vec![Cell::Empty; props.num_rows]; props.num_cols]);
    let player_turn = use_state(move || initial_turn);
    let game_state = use_state(|| GameState::Ongoing);

    let toggle_color_blind_mode = {
        let color_blind_mode = color_blind_mode.clone();
        Callback::from(move |_| {
            color_blind_mode.set(if *color_blind_mode == ColorBlindMode::On { ColorBlindMode::Off } else { ColorBlindMode::On });
        })
    };

    html! {
        <>
            <button
                onclick={toggle_color_blind_mode}
                class="mt-3 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {format!("Color Blind Mode: {}", color_blind_mode.to_string())}
            </button>
            <div class={format!("{}", if *color_blind_mode==ColorBlindMode::On { "board-color-blind" } else { "board" })} style={format!("grid-template-columns: repeat({},50px)", props.num_cols)}>
                { for (0..props.num_cols).map(|x| create_column(x, props.num_rows, board.clone(), player_turn.clone(), game_state.clone(), color_blind_mode.clone(), game_difficulty.clone())) }
            </div>
            <div class="h-8 flex items-center">
                <p class="font-bold">
                    {
                        match *game_state {
                            GameState::WonBy(Cell::X) => "You win!".to_string(),
                            GameState::WonBy(Cell::O) => "Bot wins!".to_string(),
                            GameState::Draw => "Draw!".to_string(),
                            _ => "".to_string(),
                        }
                    }
                </p>
            </div>

        </>
    }
}

fn create_column(
    x: usize,
    num_rows: usize,
    board: UseStateHandle<Vec<Vec<Cell>>>,
    player_turn: UseStateHandle<Cell>,
    game_state: UseStateHandle<GameState>,
    color_blind_mode: UseStateHandle<ColorBlindMode>,
    game_difficulty: UseStateHandle<Difficulty>
) -> Html {
    let onclick = {
        let board = board.clone();
        let player_turn = player_turn.clone();
        let game_state = game_state.clone();
        Callback::from(move |_| {
            if matches!(*game_state, GameState::Ongoing) {
                let mut new_board = (*board).clone();
                let column_filled = new_board[x].iter().all(|cell| *cell != Cell::Empty);
                if !column_filled {
                    for y in (0..num_rows).rev() {
                        if matches!(new_board[x][y], Cell::Empty) {
                            new_board[x][y] = (*player_turn).clone();
                            
                            if check_for_win(&new_board) {
                                board.set(new_board.clone());
                                game_state.set(GameState::WonBy((*player_turn).clone()));
                            } else {
                                if let Some(computer_board) = make_computer_move(&new_board, &player_turn, &game_state, &game_difficulty) {
                                    // Check if the computer's move resulted in a win.
                                    if check_for_win(&computer_board) {
                                        game_state.set(GameState::WonBy(match *player_turn {
                                            // Assuming the computer plays the opposite of the player's turn
                                            Cell::X => Cell::O,
                                            Cell::O => Cell::X,
                                            _ => unreachable!(),
                                        }));
                                    } else {
                                        if computer_board.iter().all(|col| col[0] != Cell::Empty) {
                                            game_state.set(GameState::Draw);
                                        }
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
        <div class="column" {onclick} style={format!("cursor: {}", if *game_state==GameState::Ongoing { "pointer" } else { "" })}>
            { for (0..num_rows).map(|y| {
                let cell = board[x][y].clone();
                let symbol = match cell {
                    Cell::X => "X",
                    Cell::O => "O",
                    Cell::Empty => "",
                };
                
                match *color_blind_mode {
                    ColorBlindMode::Off => {
                        html! {
                            <div class="cell">
                                if symbol == "X"{
                                    <div class="circle-red">
                                    </div>
                                }
                                if symbol == "O"{
                                    <div class="circle-yellow">
                                    </div>
                                }
                                if symbol == ""{
                                    <div class="circle-white">
                                    </div>
                                }
                            </div>
                        }
                    }
                    ColorBlindMode::On => {
                        html! {
                            <div class="cell-colorblind">
                                if symbol == "X"{
                                    <div class="circle-green-colorblind">
                                    </div>
                                }
                                if symbol == "O"{
                                    <div class="circle-yellow-colorblind">
                                    </div>
                                }
                                if symbol == ""{
                                    <div class="circle-white">
                                    </div>
                                }
                            </div>
                        }
                    }
                }
            })}
        </div>
    }
}

fn check_for_win(board: &Vec<Vec<Cell>>) -> bool {

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
    for y in 0..rows {
        for x in 0..cols {
            for sequence in &win_sequences {
                // Down-right
                if x <= cols - 4 && y <= rows - 4 && (0..4).all(|i| board[y + i][x + i] == sequence[i]) {
                    return true;
                }
                // Up-right
                if x <= cols - 4 && y >= 3 && (0..4).all(|i| board[y - i][x + i] == sequence[i]) {
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
    game_difficulty: &Difficulty
) -> Option<Vec<Vec<Cell>>> {
    if matches!(*game_state, GameState::Ongoing) {

        let mut rng = thread_rng();
        let rows = board[0].len();
        let cols = board.len();

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
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) {
                            let mut new_board = board.clone();
                            new_board[col][row] = computer_cell;
                            return Some(new_board);
                        }
                    }
                }
            },
            Difficulty::Hard => {
                // Determine the computer's and player's cell types
                let computer_cell =  match player_turn {
                    Cell::X => Cell::O,
                    Cell::O => Cell::X,
                    _ => unreachable!(),
                };

                // Try to find a winning move for the computer
                for col in 0..cols {
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) {
                            let mut temp_board = board.clone();
                            temp_board[col][row] = Cell::O;
                            if check_for_win(&temp_board) {
                                return Some(temp_board);
                            }
                            break; // Move to the next column after checking the bottom-most empty cell
                        }
                    }
                }

                // Try to block the player's winning move
                for col in 0..cols {
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) {
                            let mut temp_board = board.clone();
                            temp_board[col][row] = Cell::X; // Temporarily simulate the player's move
                            if check_for_win(&temp_board) {
                                temp_board[col][row] = Cell::O; // Block the player's win
                                return Some(temp_board);
                            }
                            break; // Move to the next column after checking the bottom-most empty cell
                        }
                    }
                }

                // Attempt to place the computer's piece in a random column
                for _ in 0..cols {
                    let col = rng.gen_range(0..cols);
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) {
                            let mut new_board = board.clone();
                            new_board[col][row] = computer_cell;
                            return Some(new_board);
                        }
                    }
                }
            },
        }
    }

    None
}
