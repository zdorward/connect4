use yew::prelude::*;
use yew::{function_component, html};

#[function_component]
pub fn TootAndOttoRules() -> Html {
    html! {
        <div class="max-w-4xl mx-auto p-5">
            <h1 class="text-3xl font-bold text-center text-gray-800 mb-4">{"Toot and Otto Rules"}</h1>
            <p class="text-gray-600 text-lg">{"Toot and Otto is a two-player game. The game is played on a 3x3 grid. The players take turns to place their mark on the grid. The first player to get three of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            <h2 class="text-2xl font-semibold text-gray-700 mt-5 mb-3">{"How to play"}</h2>
            <p class="text-gray-600 text-lg">{"Toot and Otto is played on a 6 col x 4 row grid. The players take turns to place their mark on the grid. The first player to get three of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            <h2 class="text-2xl font-semibold text-gray-700 mt-5 mb-3">{"How to win"}</h2>
            <p class="text-gray-600 text-lg">{"The first player to get three of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            <h2 class="text-2xl font-semibold text-gray-700 mt-5 mb-3">{"How to draw"}</h2>
            <p class="text-gray-600 text-lg">{"If all the cells on the grid are filled and no player has three of their marks in a row, the game is a draw."}</p>
        </div>
    }
}

#[function_component]
pub fn Connect4Rules() -> Html {
    html!{
        <div class="max-w-4xl mx-auto p-5">
            <h1 class="text-3xl font-bold text-center text-gray-800 mb-4">{"Connect 4 Rules"}</h1>
            <p class="text-gray-600 text-lg">{"Connect 4 is a two-player game. The game is played on a 6x7 grid. The players take turns to place their mark on the grid. The first player to get four of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            <h2 class="text-2xl font-semibold text-gray-700 mt-5 mb-3">{"How to play"}</h2>
            <p class="text-gray-600 text-lg">{"Connect 4 is played on a 7 col x 6 row grid. The players take turns to place their mark on the grid. The first player to get four of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            <h2 class="text-2xl font-semibold text-gray-700 mt-5 mb-3">{"How to win"}</h2>
            <p class="text-gray-600 text-lg">{"The first player to get four of their marks in a row (horizontally, vertically, or diagonally) wins the game."}</p>
            <h2 class="text-2xl font-semibold text-gray-700 mt-5 mb-3">{"How to draw"}</h2>
            <p class="text-gray-600 text-lg">{"If all the cells on the grid are filled and no player has four of their marks in a row, the game is a draw."}</p>
        </div>
    }
}
