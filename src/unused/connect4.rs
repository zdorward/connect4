use yew::prelude::*;
use yew::{function_component, html};

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

enum GameState {
    Ongoing,
    WonBy(Cell),
}

#[function_component(Connect4BoardEric)]
pub fn connect_4_board() -> Html {
    let board = use_state(|| vec![vec![Cell::Empty; 6]; 7]);
    let player_turn = use_state(|| Cell::X);
    let game_state = use_state(|| GameState::Ongoing);

    html! {
        <>
            <div class="board">
                { for (0..7).map(|x| {
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
                                let column_filled = new_board[x].iter().all(|cell| matches!(cell, Cell::O) || matches!(cell, Cell::X));
                                if !column_filled {
                                    for y in (0..6).rev() {
                                        if matches!(new_board[x][y], Cell::Empty) {
                                            new_board[x][y] = (*player_turn).clone();
                                            
                                            if check_for_win(&new_board) {
                                                game_state.set(GameState::WonBy((*player_turn).clone()));
                                                break;
                                            }
                                            
                                            player_turn.set(match *player_turn {
                                                Cell::X => Cell::O,
                                                Cell::O => Cell::X,
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
                            { for (0..6).map(|y| {
                                let cell = board[x][y].clone();
                                let symbol = match cell {
                                    Cell::X => "X",
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
                        GameState::WonBy(Cell::X) => "Player X won!".to_string(),
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

    // Check horizontal lines
    for y in 0..rows {
        for x in 0..cols - 3 {
            if let Some(cell) = board[y][x].clone().into_option() {
                if board[y][x + 1] == cell && board[y][x + 2] == cell && board[y][x + 3] == cell {
                    return true;
                }
            }
        }
    }

    // Check vertical lines
    for x in 0..cols {
        for y in 0..rows - 3 {
            if let Some(cell) = board[y][x].clone().into_option() {
                if board[y + 1][x] == cell && board[y + 2][x] == cell && board[y + 3][x] == cell {
                    return true;
                }
            }
        }
    }

    // Check diagonal (down-right and up-right)
    for y in 0..rows - 3 {
        for x in 0..cols - 3 {
            if let Some(cell) = board[y][x].clone().into_option() {
                // Down-right
                if board[y + 1][x + 1] == cell && board[y + 2][x + 2] == cell && board[y + 3][x + 3] == cell {
                    return true;
                }
                // Up-right (for diagonals going the other way, we start from the bottom)
                if y >= 3 && board[y - 1][x + 1] == cell && board[y - 2][x + 2] == cell && board[y - 3][x + 3] == cell {
                    return true;
                }
            }
        }
    }

    false
}

// Helper to convert Cell to Option<Cell> for easier checking
impl Cell {
    fn into_option(self) -> Option<Self> {
        match self {
            Cell::Empty => None,
            _ => Some(self),
        }
    }
}