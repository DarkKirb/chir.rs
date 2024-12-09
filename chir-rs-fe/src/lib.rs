//! Management frontend for chir.rs

use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use wasm_bindgen::prelude::*;
use wasm_tracing::{WASMLayer, WASMLayerConfigBuilder};
use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;
pub mod login;

/// Routes defined by this SPA
#[derive(Clone, Routable, PartialEq)]
enum Route {
    /// Home page
    #[at("/")]
    Home,
    /// Login page
    #[at("/login")]
    Login,
}

/// Render the main site content
#[allow(clippy::needless_pass_by_value, reason = "API reasons")]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home::home_page(),
        Route::Login => login::login(),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="container">
            <header>
                <hgroup>
                    <h1> {"Lotte’s Attic"} </h1>
                    <p> {"Super secret raccministrator portal :3c "} </p>
                </hgroup>
            </header>
            <main id="main" role="main">
                <HashRouter>
                    <Switch<Route> render={switch} />
                </HashRouter>
            </main>
        </div>
    }
}

#[wasm_bindgen(start)]
fn main() {
    console_error_panic_hook::set_once();
    tracing_subscriber::registry()
        .with(WASMLayer::new(WASMLayerConfigBuilder::new().build()))
        .init();

    yew::Renderer::<App>::new().render();
}
