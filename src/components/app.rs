use yew::prelude::*;
use crate::components::rules::{TootAndOttoRules, Connect4Rules};
use crate::components::connect4::Connect4Board;
use crate::components::toototto::TootOttoBoard;
use crate::components::game_types::{
    GameVersion, 
    GameVersion::Connect4, 
    GameVersion::TootOtto, 
    Difficulty::Easy, 
    Difficulty::Hard
};

#[function_component]
pub fn App() -> Html {
    let game_version = use_state(|| Connect4);
    let restart_counter = use_state(|| 0); // State to trigger board restarts

    let difficulty = use_state(|| Easy); // Adding a state for the difficulty

    let toggle_version = {
        let game_version = game_version.clone();
        Callback::from(move |_| {
            game_version.set(if *game_version == Connect4 {
                TootOtto
            } else {
                Connect4
            });
        })
    };

    let restart_game = {
        let restart_counter = restart_counter.clone();
        Callback::from(move |_| {
            restart_counter.set(*restart_counter + 1); // Increment the counter to trigger a re-render
        })
    };

    let toggle_difficulty = {
        let difficulty = difficulty.clone();
        Callback::from(move |_| {
            difficulty.set(if *difficulty == Easy { Hard } else { Easy });
        })
    };

    let render_game = |version: &GameVersion| {
        match version {
            Connect4 => html! {
                <>
                    <Connect4Rules/>
                    <Connect4Board key={format!("connect4-{}-{}", *restart_counter, (*difficulty).to_string())} difficulty={(*difficulty).clone()}/>
                </>
            },
            TootOtto => html! {
                <>
                    <TootAndOttoRules/>
                    <TootOttoBoard key={format!("toototto-{}-{}", *restart_counter, (*difficulty).to_string())} difficulty={(*difficulty).clone()}/>
                </>
            },
        }
    };

    html! {
        <>
            <h1 class="text-5xl md:text-6xl font-bold text-center text-gray-800 my-8">{"Group 5's Project 3"}</h1>
            { render_game(&game_version) }
            <button
                onclick={toggle_version}
                class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {"Switch Game Version"}
            </button>
            <button
                onclick={toggle_difficulty}
                class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {difficulty.to_string()}
            </button>
            <button
                onclick={restart_game}
                class="mt-4 mb-40 ml-4 py-2 px-4 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {"Restart Game"}
            </button>
        </>
    }
}
