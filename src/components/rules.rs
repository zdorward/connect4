use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Connect4RuleProps {
    pub on_connect4_click: Callback<()>,
}
#[derive(Properties, PartialEq)]
pub struct TootOttoRuleProps {
    pub on_toototto_click: Callback<()>,
}

#[function_component(Connect4Rules)]
pub fn connect4_rules(props: &Connect4RuleProps) -> Html {
    let onclick = {
        let on_button_click = props.on_connect4_click.clone();
        Callback::from(move |_| on_button_click.emit(()))
    };

    html!{
        <div class="max-w-4xl mx-auto p-5">
            <h1 class="text-3xl font-bold text-center text-gray-800 mb-4">{"Connect 4 Rules"}</h1>
            <p>{"Connect 4 is a two-player connection game in which the players first choose a color and then take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The pieces fall straight down, occupying the lowest available space within the column. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}</p>

            <button 
                {onclick} 
                class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                >
            {"Play Connect 4"}</button>
        </div>
    }
}

#[function_component(TootAndOttoRules)]
pub fn toot_and_otto_rules(props: &TootOttoRuleProps) -> Html {
    let onclick = {
        let on_button_click = props.on_toototto_click.clone();
        Callback::from(move |_| on_button_click.emit(()))
    };

    html! {
        <div class="max-w-4xl mx-auto p-5">
            <h1 class="text-3xl font-bold text-center text-gray-800 mb-4">{"Toot and Otto Rules"}</h1>
            <p>{"Toot and Otto is a two-player strategy game where players take turns drawing lines between dots on a grid. The goal is to form squares. When a player completes a square, they write their initial in it and take another turn. The game ends when all the squares have been completed, and the player with the most squares wins."}</p>
            <button 
                {onclick} 
                class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                >{"Play Toot and Otto"}
            </button>
        </div>
    }
}
