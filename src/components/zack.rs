use yew::prelude::*;
use yew::{function_component, html};

#[function_component(Connect4Board)]
pub fn connect_4_board() -> Html {
    html! {
        <div class="board">
            { for (0..7).map(|_| {
                html! {
                    <div class="row">
                        { for (0..6).map(|_| {
                            html! {
                                <div class="cell"></div>
                            }
                        })}
                    </div>
                }
            })}
        </div>
    }
}