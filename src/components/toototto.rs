use yew::prelude::*;
use rand::{thread_rng, Rng};
use crate::components::lib::{Difficulty, BoardProps};

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

#[function_component(TootOttoBoard)]
pub fn connect_4_board(props: &BoardProps) -> Html {
    let num_rows = 4;
    let num_cols = 6;

    let board = use_state(|| vec![vec![Cell::Empty; num_rows]; num_cols]);
    let player_choice = use_state(|| Cell::T); // Assuming player starts with 'T'
    let game_state = use_state(|| GameState::Ongoing);
    let game_difficulty = use_state(|| props.difficulty.clone());

    let player_t_count = use_state(|| 6);
    let player_o_count = use_state(|| 6);
    let bot_t_count = use_state(|| 6);
    let bot_o_count = use_state(|| 6);    

    let toggle_player_choice = {
        let player_choice = player_choice.clone();
        Callback::from(move |_| {
            let new_choice = match *player_choice {
                Cell::T => Cell::O,
                Cell::O => Cell::T,
                _ => Cell::T,
            };
            player_choice.set(new_choice);
        })
    };

    html! {
        <>
            <h1 class="text-4xl md:text-6xl font-bold text-center text-gray-800 my-8">{ format!("Toot and Otto")}</h1>
            <div> {"You are trying to spell \"Toot\" and the bot is trying to spell \"Otto\""}</div>
            <button 
                onclick={toggle_player_choice}
                class="mt-4 mb-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                { format!("Selected Piece: {}", match *player_choice { Cell::T => "T", Cell::O => "O", _ => "Error" }) }
            </button>
            
            <div class="flex justify-around items-center mt-4">
                <div class="text-center p-4 bg-gray-100 rounded-lg shadow">
                    {format!("Player T's left: {}", *player_t_count)}
                </div>
                <div class="text-center p-4 bg-gray-100 rounded-lg shadow">
                    {format!("Player O's left: {}", *player_o_count)}
                </div>
                <div class="text-center p-4 bg-gray-100 rounded-lg shadow">
                    {format!("Bot T's left: {}", *bot_t_count)}
                </div>
                <div class="text-center p-4 bg-gray-100 rounded-lg shadow">
                    {format!("Bot O's left: {}", *bot_o_count)}
                </div>
            </div>

            <div class="board">
                { for (0..num_cols).map(|x| create_column(x, num_rows, board.clone(), player_choice.clone(), game_state.clone(), game_difficulty.clone(), player_t_count.clone(), player_o_count.clone(), bot_t_count.clone(), bot_o_count.clone())) }
            </div>
            <p>
                {
                    match *game_state {
                        GameState::WonBy(Cell::T) => "You win!".to_string(),
                        GameState::WonBy(Cell::O) => "Bot wins!".to_string(),
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
    player_choice: UseStateHandle<Cell>,
    game_state: UseStateHandle<GameState>,
    game_difficulty: UseStateHandle<Difficulty>,
    player_t_count: UseStateHandle<i32>,
    player_o_count: UseStateHandle<i32>,
    bot_t_count: UseStateHandle<i32>,
    bot_o_count: UseStateHandle<i32>
) -> Html {
    let onclick = {
        let board = board.clone();
        let player_choice = player_choice.clone();
        let game_state = game_state.clone();
        let game_difficulty = game_difficulty.clone();
        Callback::from(move |_| {
            if matches!(*game_state, GameState::Ongoing) {
                let mut new_board = (*board).clone();
                if let Some(updated_board) = make_player_move(x, num_rows, &new_board, &player_choice, &player_t_count, &player_o_count) {
                    new_board = updated_board;
                    let win_state = check_for_win(&new_board);
                    if win_state != Cell::Empty {
                        game_state.set(GameState::WonBy(win_state));
                    } else {
                        // Computer's turn to play after player's move
                        if let Some(computer_board) = make_computer_move(&new_board, &game_state, &game_difficulty, &bot_t_count, &bot_o_count) {
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
    player_choice: &Cell,
    player_t_count: &UseStateHandle<i32>,
    player_o_count: &UseStateHandle<i32>
) -> Option<Vec<Vec<Cell>>> {
    let mut new_board = board.clone();
    let column_filled = new_board[x].iter().all(|cell| !matches!(cell, Cell::Empty));
    if !column_filled {
        for y in (0..num_rows).rev() {
            if matches!(new_board[x][y], Cell::Empty) {
                match player_choice {
                    Cell::T if **player_t_count > 0 => {
                        new_board[x][y] = Cell::T;
                        player_t_count.set(**player_t_count - 1);
                    },
                    Cell::O if **player_o_count > 0 => {
                        new_board[x][y] = Cell::O;
                        player_o_count.set(**player_o_count - 1);
                    },
                    _ => return None,
                }
                return Some(new_board);
            }
        }
    }
    None
}


fn make_computer_move(
    board: &Vec<Vec<Cell>>,
    game_state: &GameState,
    game_difficulty: &Difficulty,
    bot_t_count: &UseStateHandle<i32>,
    bot_o_count: &UseStateHandle<i32>
) -> Option<Vec<Vec<Cell>>> {
    if matches!(*game_state, GameState::Ongoing) {

        let mut rng = thread_rng();
        let rows = board[0].len();
        let cols = board.len();

        match game_difficulty {
            Difficulty::Easy => {
                // Determine the computer's cell type based on the current player's type and game version
                let computer_cell = if rng.gen_bool(0.5) { Cell::T } else { Cell::O };

                for _ in 0..cols {
                    let col = rng.gen_range(0..cols);
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) {
                            let mut new_board = board.clone();

                            // Inside each condition where the bot places a 'T' or 'O':
                            if matches!(computer_cell, Cell::T) && **bot_t_count > 0 {
                                new_board[col][row] = Cell::T;
                                bot_t_count.set(**bot_t_count - 1);
                            } else if matches!(computer_cell, Cell::O) && **bot_o_count > 0 {
                                new_board[col][row] = Cell::O;
                                bot_o_count.set(**bot_o_count - 1);
                            }

                            return Some(new_board);
                        }
                    }
                }

            },
            Difficulty::Hard => {
                // Try to find a winning move for the computer using the letter O
                for col in 0..cols {
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) && **bot_o_count > 0 {
                            let mut temp_board = board.clone();
                            temp_board[col][row] = Cell::O;
                            if let Cell::O = check_for_win(&temp_board) {
                                bot_o_count.set(**bot_o_count - 1);
                                return Some(temp_board);
                            }
                            break; // Move to the next column after checking the bottom-most empty cell
                        }
                    }
                }

                // Try to find a winning move for the computer the letter T
                for col in 0..cols {
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) && **bot_t_count > 0 {
                            let mut temp_board = board.clone();
                            temp_board[col][row] = Cell::T;
                            if let Cell::O = check_for_win(&temp_board) {
                                bot_t_count.set(**bot_t_count - 1);
                                return Some(temp_board);
                            }
                            break; // Move to the next column after checking the bottom-most empty cell
                        }
                    }
                }

                // Try to block the player's winning move using a T
                for col in 0..cols {
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) && **bot_t_count > 0 {
                            let mut temp_board = board.clone();
                            temp_board[col][row] = Cell::O; // Temporarily simulate the player's move
                            if let Cell::T = check_for_win(&temp_board) {
                                temp_board[col][row] = Cell::T; // Block the player's win
                                bot_t_count.set(**bot_t_count - 1);
                                return Some(temp_board);
                            }
                            break; // Move to the next column after checking the bottom-most empty cell
                        }
                    }
                }

                // Try to block the player's winning move using a O
                for col in 0..cols {
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) && **bot_o_count > 0 {
                            let mut temp_board = board.clone();
                            temp_board[col][row] = Cell::T; // Temporarily simulate the player's move
                            if let Cell::T = check_for_win(&temp_board) {
                                temp_board[col][row] = Cell::O; // Block the player's win
                                bot_o_count.set(**bot_o_count - 1);
                                return Some(temp_board);
                            }
                            break; // Move to the next column after checking the bottom-most empty cell
                        }
                    }
                }

                let computer_cell = if rng.gen_bool(0.5) { Cell::T } else { Cell::O };

                // Attempt to place the computer's piece in a random column
                for _ in 0..cols {
                    let col = rng.gen_range(0..cols);
                    for row in (0..rows).rev() {
                        if matches!(board[col][row], Cell::Empty) {
                            let mut new_board = board.clone();
                            if matches!(computer_cell, Cell::T) {
                                bot_t_count.set(**bot_t_count - 1);
                            } else {
                                bot_o_count.set(**bot_o_count - 1);
                            }

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
