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
            <button {onclick} class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-700">{"Play Connect 4"}</button>
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
            <button {onclick} class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-700">{"Play Toot and Otto"}</button>
        </div>
    }
}
