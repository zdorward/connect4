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
        <div class="max-w-4xl mx-auto p-5 flex flex-col items-center">
            <h1 class="text-3xl font-bold text-center text-gray-800 mb-4">{"Connect 4 Rules"}</h1>
            <p>{"Connect 4 is a two-player game where players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The pieces fall straight down, occupying the lowest available space within the column. The objective is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}</p>

            <button 
                {onclick} 
                class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {"Play Connect 4"}
            </button>
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
        <div class="max-w-4xl mx-auto p-5 flex flex-col items-center">
            <h1 class="text-3xl font-bold text-center text-gray-800 mb-4">{"TOOT and OTTO Rules"}</h1>
            <p>{"TOOT and OTTO is a two-player word-building game. Players take turns placing either a 'T' or an 'O' on a grid, aiming to spell out 'TOOT' (Player 1) or 'OTTO' (Player 2). Like Connect 4, the pieces fall straight down, occupying the lowest available space within the column.  Each letter can only be used six times, and players aim to create their words vertically, horizontally, or diagonally."}</p>
            <button 
                {onclick} 
                class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {"Play TOOT and OTTO"}
            </button>
        </div>
    }
}
