use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::rules::{TootAndOttoRules, Connect4Rules};
use crate::components::connect4::Connect4Board;
use crate::components::toototto::TootOttoBoard;
use crate::components::lib::ColorBlindMode;
use crate::components::lib::{
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
    let colorblind_mode = use_state(|| ColorBlindMode::Off); // Adding a state for the difficulty
    let num_rows = use_state(||6); // Default 6 rows
    let num_cols = use_state(||7); // Default 7 columns

    let toggle_color_mode = {
        let colorblind_mode = colorblind_mode.clone();
        Callback::from(move |_| {
            colorblind_mode.set(if *colorblind_mode == ColorBlindMode::Off { ColorBlindMode::On } else { ColorBlindMode::Off });
        })
    };

    let set_num_rows = {
        let num_rows = num_rows.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                // Safely parse the input value and set the number of rows, default to 4 if parsing fails
                num_rows.set(input.value().parse::<usize>().unwrap_or(4));
            }
        })
    };
    
    let set_num_cols = {
        let num_cols = num_cols.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                // Safely parse the input value and set the number of columns, default to 4 if parsing fails
                num_cols.set(input.value().parse::<usize>().unwrap_or(4));
            }
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

    let show_rules_from_game = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.set(AppState::ShowRules);
        })
    };

    let game_buttons = html! {
        <>
            // <div style="display: flex; flex-direction: column; gap: 10px; margin-top: 10px">
                <button
                    onclick={toggle_difficulty}
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                >
                    {format!("Difficulty: {}", difficulty.to_string())}
                </button>
                <button
                    onclick={restart_game}
                    class="mt-3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                >
                    {"Restart Game"}
                </button>
                <button 
                    onclick={show_rules_from_game} 
                    class="mt-3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                >
                    { "See Rules" }
                </button>
            // </div>
        </>    
    };

    let connect4_rules = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.set(AppState::PlayConnect4);
        })
    };

    let toot_otto_rules = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.set(AppState::PlayTootOtto);
        })
    };

    let render_game = |state: &AppState| {
        match state {
            AppState::MainMenu => html! {
                <>
                    <div class="text-5xl mt-10 mb-44"> 
                        {"Group 5 Project 3"} 
                    </div>
                    <div class="main-menu">
                        <button 
                            onclick={switch_to_connect4} 
                            class="mt-3 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-3xl text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        >
                            { "Play Connect4" }
                        </button>
                        <button 
                            onclick={switch_to_toot_otto} 
                            class="mt-3 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-3xl text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        >
                            { "Play TOOT and OTTO" }
                        </button>
                        <button 
                            onclick={show_rules} 
                            class="mt-4 py-2 px-4 bg-blue-500 hover:bg-blue-700 text-3xl text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        >
                            { "See Rules" }
                        </button>
                    </div>
                </>
            },
            AppState::PlayConnect4 => html! {
                <>
                    
                    <h1 class="text-4xl  font-bold text-center text-gray-800 mt-4 mb-4">{ format!("Connect 4") }</h1>

                    <div class="inputdiv">
                    <div class="inputdiv">
                        <h1 class="text-large text-center text-gray-800">{ format!("Rows") }</h1>
                        <input class="input" type="number" min="4" max="20" value={(*num_rows).to_string()} oninput={set_num_rows} />
                    </div>
                    <div class="inputdiv">
                        <h1 class="text-large text-center text-gray-800">{ format!("Columns") }</h1>
                        <input class="input" type="number" min="4" max="20" value={(*num_cols).to_string()} oninput={set_num_cols} />
                    </div>
                    </div>
                    <Connect4Board 
                        key={format!("board-{}-{}-{}-{}", *num_rows, *num_cols, *restart_counter, (*difficulty).to_string())}
                        difficulty={(*difficulty).clone()} 
                        num_rows={*num_rows} 
                        num_cols={*num_cols}
                        colorblind_mode={(*colorblind_mode).clone()}
                        on_color_blind_mode_toggle={toggle_color_mode}
                    />
                    { game_buttons }
                    <button
                        onclick={switch_to_toot_otto}
                        class="mt-4 py-2 px-4 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                    {"Play TOOT and OTTO"}
                    </button>
                </>
            },
            AppState::PlayTootOtto => html! {
                <>
                    <h1 class="text-4xl font-bold text-center text-gray-800 my-4">{ format!("TOOT and OTTO")}</h1>
                    
                    <TootOttoBoard 
                        key={format!("toototto-{}-{}", *restart_counter, *difficulty)} 
                        difficulty={(*difficulty).clone()}
                        num_rows={*num_rows} 
                        num_cols={*num_cols}
                    />
                    { game_buttons }
                    <button
                        onclick={switch_to_connect4}
                        class="mt-4 py-2 px-4 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                    {"Play Connect 4"}
                    </button>
                </>            },
            AppState::ShowRules => html! {
                <>
                    <Connect4Rules on_connect4_click={connect4_rules}/>
                    <TootAndOttoRules on_toototto_click={toot_otto_rules}/>
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

