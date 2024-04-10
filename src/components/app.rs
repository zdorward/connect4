
use yew::prelude::*;
use crate::components::rules::{TootAndOttoRules, Connect4Rules};
use crate::components::connect4::{Connect4BoardEric};
use crate::components::zack::{Connect4Board};

// mod connect4;
// mod tootandotto;

#[function_component]
pub fn App() -> Html {

    html! {
        <>
            <h1>{"Connect 4"}</h1>
            <Connect4Rules/>
            <TootAndOttoRules/>
            <Connect4BoardEric/>
            <Connect4Board/>
        </>
    }
}
