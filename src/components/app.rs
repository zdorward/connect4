
use yew::prelude::*;
use crate::components::rules::{TootAndOttoRules, Connect4Rules};
use crate::components::connect4::{Connect4BoardEric};
use crate::components::toototto::{TootBoard};
// use crate::components::toottest::{TootTest};

// mod connect4;
// mod tootandotto;

#[function_component]
pub fn App() -> Html {

    html! {
        <>
        <h1 class="text-5xl md:text-6xl font-bold text-center text-gray-800 my-8">{"Group 5's Project 3"}</h1>
            <Connect4Rules/>
            <Connect4BoardEric/>

            <TootAndOttoRules/>
            <TootBoard/>
            <TootAndOttoRules/>
        </>
    }
}
