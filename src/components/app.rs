use yew::prelude::*;
use crate::components::rules::{TootAndOttoRules, Connect4Rules};
use crate::components::connect4::Connect4Board;
use crate::components::toototto::TootOttoBoard;
use crate::components::lib::{
    GameVersion, 
    GameVersion::Connect4, 
    GameVersion::TootOtto, 
    Difficulty::Easy, 
    Difficulty::Hard
};

enum AppState {
    MainMenu,
    PlayConnect4,
    PlayTootOtto,
    ShowRules,
}


#[function_component]
pub fn App() -> Html {
    let app_state = use_state(|| AppState::MainMenu);
    let restart_counter = use_state(|| 0); // State to trigger board restarts
    let difficulty = use_state(|| Easy); // Adding a state for the difficulty

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

    let switch_to_connect4 = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.set(AppState::PlayConnect4);
        })
    };

    let switch_to_toot_otto = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.set(AppState::PlayTootOtto);
        })
    };

    let show_rules = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.set(AppState::ShowRules);
        })
    };

    let buttons = html! {
        <>
            <button
                onclick={toggle_difficulty}
                class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {difficulty.to_string()}
            </button>
            <button
                onclick={restart_game}
                class="mt-4 py-2 px-4 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            >
                {"Restart Game"}
            </button>
        </>    
    };

    let render_game = |state: &AppState| {
        match state {
            AppState::MainMenu => html! {
                <>
                    <button onclick={switch_to_connect4} class="button">{ "Play Connect4" }</button>
                    <button onclick={switch_to_toot_otto} class="button">{ "Play Toot and Otto" }</button>
                    <button onclick={show_rules} class="button">{ "See Rules" }</button>
                </>
            },
            AppState::PlayConnect4 => html! {
                <>
                    <Connect4Board 
                        key={format!("connect4-{}-{}", *restart_counter, *difficulty)} 
                        difficulty={(*difficulty).clone()} 
                    />
                    { buttons }
                    <button
                        onclick={switch_to_toot_otto}
                        class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                    {"Play Toot and Otto"}
                    </button>
                </>
            },
            AppState::PlayTootOtto => html! {
                <>
                    <TootOttoBoard 
                        key={format!("toototto-{}-{}", *restart_counter, *difficulty)} 
                        difficulty={(*difficulty).clone()} 
                    />
                    { buttons }
                    <button
                        onclick={switch_to_connect4}
                        class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                    {"Play Connect 4"}
                    </button>
                </>            },
            AppState::ShowRules => html! {
                <>
                    <Connect4Rules />
                    <TootAndOttoRules />
                </>
            }
        }
    };

    html! {
        <>
            {render_game(&app_state)}
        </>
    }
}

