// use yew::prelude::*;
// use yew::{function_component, html};

// #[derive(Clone, PartialEq)]
// enum Cell {
//     Empty,
//     X,
//     O,
// }

// enum GameState {
//     Ongoing,
//     WonBy(Cell),
// }

// #[derive(Clone, PartialEq)]
// enum Difficulty {
//     Easy,
//     Hard,
// }

// #[function_component(Connect4Board)]
// pub fn connect_4_board() -> Html {
//     let board = use_state(|| vec![vec![Cell::Empty; 6]; 7]);
//     let player_turn = use_state(|| Cell::X);
//     let game_state = use_state(|| GameState::Ongoing);
//     let difficulty = use_state(|| Difficulty::Easy); // Default to Easy mode for simplicity

//     // Function to make a move for the bot in Easy mode
//     let make_bot_move = {
//         let board = board.clone();
//         let player_turn = player_turn.clone();
//         let game_state = game_state.clone();
//         Callback::from(move |_| {
//             if matches!(*game_state, GameState::Ongoing) && matches!(*player_turn, Cell::O) {
//                 let mut rng = thread_rng();
//                 loop {
//                     let x = rng.gen_range(0..7);
//                     let mut new_board = (*board).clone();
//                     let column_filled = new_board[x].iter().all(|cell| matches!(cell, Cell::O) || matches!(cell, Cell::X));
//                     if !column_filled {
//                         for y in (0..6).rev() {
//                             if matches!(new_board[x][y], Cell::Empty) {
//                                 new_board[x][y] = Cell::O;
//                                 if check_for_win(&new_board) {
//                                     game_state.set(GameState::WonBy(Cell::O));
//                                 } else {
//                                     player_turn.set(Cell::X); // Switch back to player X
//                                 }
//                                 board.set(new_board);
//                                 break;
//                             }
//                         }
//                         break; // Exit the loop after making a move
//                     }
//                 }
//             }
//         })
//     };

//     let on_easy_click = {
//         let difficulty = difficulty.clone();
//         Callback::from(move |_| difficulty.set(Difficulty::Easy))
//     };

//     let on_hard_click = {
//         let difficulty = difficulty.clone();
//         Callback::from(move |_| difficulty.set(Difficulty::Hard))
//     };

//     html! {
//         <>
//             <button {on_easy_click}>{"Easy"}</button>
//             <button {on_hard_click}>{"Hard"}</button>
//             <p>{format!("Current mode: {:?}", *difficulty)}</p>
//             <div class="board">
//                 { for (0..7).map(|x| {
//                     let board = board.clone();
//                     let player_turn = player_turn.clone();
//                     let game_state = game_state.clone();
//                     let difficulty = difficulty.clone();
//                     let make_bot_move = make_bot_move.clone();
//                     let onclick = {
//                         let board = board.clone();
//                         let player_turn = player_turn.clone();
//                         let game_state = game_state.clone();
//                         let make_bot_move = make_bot_move.clone();
//                         Callback::from(move |_| {
//                             if matches!(*game_state, GameState::Ongoing) && matches!(*player_turn, Cell::X) {
//                                 let mut new_board = (*board).clone();
//                                 let column_filled = new_board[x].iter().all(|cell| matches!(cell, Cell::O) || matches!(cell, Cell::X));
//                                 if !column_filled {
//                                     for y in (0..6).rev() {
//                                         if matches!(new_board[x][y], Cell::Empty) {
//                                             new_board[x][y] = Cell::X;
//                                             if check_for_win(&new_board) {
//                                                 game_state.set(GameState::WonBy(Cell::X));
//                                             } else {
//                                                 player_turn.set(Cell::O); // Switch to player O
//                                                 if matches!(*difficulty, Difficulty::Easy) {
//                                                     make_bot_move.emit(()); // Make the bot move immediately after player X
//                                                 }
//                                             }
//                                             board.set(new_board);
//                                             break;
//                                         }
//                                     }
//                                 }
//                             }
//                         })
//                     };
//                     html! {
//                         <div class="column" {onclick}>
//                             { for (0..6).map(|y| {
//                                 let cell = board[x][y].clone();
//                                 let symbol = match cell {
//                                     Cell::X => "X",
//                                     Cell::O => "O",
//                                     Cell::Empty => "",
//                                 };
//                                 html! {
//                                     <div class="cell">
//                                         {symbol}
//                                     </div>
//                                 }
//                             })}
//                         </div>
//                     }
//                 })}
//             </div>
//             <p>
//                 {
//                     match *game_state {
//                         GameState::WonBy(Cell::X) => "Player X won!".to_string(),
//                         GameState::WonBy(Cell::O) => "Player O won!".to_string(),
//                         _ => "".to_string(),
//                     }
//                 }
//             </p>
//         </>
//     }
// }


// fn check_for_win(board: &Vec<Vec<Cell>>) -> bool {
//     let rows = board.len();
//     let cols = board[0].len();

//     // Check horizontal lines
//     for y in 0..rows {
//         for x in 0..cols - 3 {
//             if let Some(cell) = board[y][x].clone().into_option() {
//                 if board[y][x + 1] == cell && board[y][x + 2] == cell && board[y][x + 3] == cell {
//                     return true;
//                 }
//             }
//         }
//     }

//     // Check vertical lines
//     for x in 0..cols {
//         for y in 0..rows - 3 {
//             if let Some(cell) = board[y][x].clone().into_option() {
//                 if board[y + 1][x] == cell && board[y + 2][x] == cell && board[y + 3][x] == cell {
//                     return true;
//                 }
//             }
//         }
//     }

//     // Check diagonal (down-right and up-right)
//     for y in 0..rows - 3 {
//         for x in 0..cols - 3 {
//             if let Some(cell) = board[y][x].clone().into_option() {
//                 // Down-right
//                 if board[y + 1][x + 1] == cell && board[y + 2][x + 2] == cell && board[y + 3][x + 3] == cell {
//                     return true;
//                 }
//                 // Up-right (for diagonals going the other way, we start from the bottom)
//                 if y >= 3 && board[y - 1][x + 1] == cell && board[y - 2][x + 2] == cell && board[y - 3][x + 3] == cell {
//                     return true;
//                 }
//             }
//         }
//     }

//     false
// }

// // Helper to convert Cell to Option<Cell> for easier checking
// impl Cell {
//     fn into_option(self) -> Option<Self> {
//         match self {
//             Cell::Empty => None,
//             _ => Some(self),
//         }
//     }
// }