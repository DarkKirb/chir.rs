//! Management frontend for chir.rs

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

#[wasm_bindgen(start)]
fn main() {
    yew::Renderer::<App>::new().render();
}
