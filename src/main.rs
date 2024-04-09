use yew::prelude::*;
// mod connect4;
// mod tootandotto;

#[function_component(App)]
fn app() -> Html {
    let css_content = include_str!("../styles.css");

    // Generate cells for the board dynamically
    let cells: Html = (0..6).map(|_| {
        html! {
            <div class="row">
                { (0..7).map(|_| {
                    html! {
                        <div class="cell"></div>
                    }
                }).collect::<Html>() }
            </div>
        }
    }).collect();

    html! {
        <>
            
            <style>{ css_content }</style>
            <h1>{"Connect 4"}</h1>
            <div class="board">
                { cells }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render(); // Render the app
    // connect4::main();
    // tootandotto::main(); 
}