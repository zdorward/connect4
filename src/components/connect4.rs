use yew::prelude::*;
use yew::{function_component, html};

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

#[function_component(Connect4BoardEric)]
pub fn connect_4_board() -> Html {
    let board = use_state(|| vec![vec![Cell::Empty; 6]; 7]);
    let player_turn = use_state(|| Cell::X);

    html! {
        <div class="board">
            { for (0..7).map(|x| html! {
                <div class="row">
                    { for (0..6).map(|y| {
                        let cell = board[x][y].clone();
                        let symbol = match cell {
                            Cell::X => "X",
                            Cell::O => "O",
                            Cell::Empty => "",
                        };
                        let board_clone = board.clone();
                        let player_turn_clone = player_turn.clone();
                        let on_click: Callback<_> = Callback::from(move |_| {
                            let mut new_board = (*board_clone).clone();
                            if matches!(new_board[x][y], Cell::Empty) {
                                new_board[x][y] = (*player_turn_clone).clone();
                                player_turn_clone.set(match *player_turn_clone {
                                    Cell::X => Cell::O,
                                    Cell::O => Cell::X,
                                    Cell::Empty => unreachable!(),
                                });
                            }
                            board_clone.set(new_board);
                        });
                        html! {
                            <div class="cell" onclick={on_click}>
                                {symbol}
                            </div>
                        }
                    })}
                </div>
            })}
        </div>
    }
}
