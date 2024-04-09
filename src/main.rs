use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let css_content = include_str!("../styles.css");
    html! {
        <>
            <style>{ css_content }</style>
            <h1 class="hello-world">{ "Hello World!" }</h1>
            <div class="test">{ "aloha" }</div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
