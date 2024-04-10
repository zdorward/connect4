use yew::prelude::*;
use yew::{function_component, html};

#[function_component]
pub fn TootAndOttoRules() -> Html {
    html! {
        <div>
            <h1>{"Toot and Otto Rules"}</h1>
            // <p>{"Toot and Otto is a two-player game. The game is played on a 3x3 grid. The players take turns to place their mark on the grid. The first player to get three of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            // <h2>{"How to play"}</h2>
            // <p>{"Toot and Otto is played on a 3x3 grid. The players take turns to place their mark on the grid. The first player to get three of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            // <h2>{"How to win"}</h2>
            // <p>{"The first player to get three of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            // <h2>{"How to draw"}</h2>
            // <p>{"If all the cells on the grid are filled and no player has three of their marks in a row, the game is a draw."}</p>
        </div>
    }
}

#[function_component]
pub fn Connect4Rules() -> Html {
    html!{
        <div>
            <h1>{"Connect 4 Rules"}</h1>
            // <p>{"Connect 4 is a two-player game. The game is played on a 6x7 grid. The players take turns to place their mark on the grid. The first player to get four of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            // <h2>{"How to play"}</h2>
            // <p>{"Connect 4 is played on a 6x7 grid. The players take turns to place their mark on the grid. The first player to get four of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            // <h2>{"How to win"}</h2>
            // <p>{"The first player to get four of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            // <h2>{"How to draw"}</h2>
            // <p>{"If all the cells on the grid are filled and no player has four of their marks in a row, the game is a draw."}</p>
        </div>
    }
}