//! Management frontend for chir.rs

use std::future::Future;

use tracing::{error, instrument};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use wasm_bindgen::prelude::*;
use wasm_tracing::{WasmLayer, WasmLayerConfig};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

pub mod home;
pub mod login;
pub mod robots;

/// Spans a future asynchronously
#[instrument(skip(fut))]
pub fn spawn<F>(fut: F)
where
    F: Future<Output = eyre::Result<()>> + 'static,
{
    spawn_local(async move {
        if let Err(e) = fut.await {
            error!("Failed to run async closure: {e:?}");
        }
    });
}

/// Routes defined by this SPA
#[derive(Clone, Routable, PartialEq)]
enum Route {
    /// Home page
    #[at("/")]
    Home,
    /// Login page
    #[at("/login")]
    Login,
    /// Robots page
    #[at("/robots")]
    Robots,
}

/// Render the main site content
#[allow(clippy::needless_pass_by_value, reason = "API reasons")]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <home::HomePage />
        },
        Route::Login => html! {
            <login::Login />
        },
        Route::Robots => html! {
            <robots::Robots />
        },
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
        .with(WasmLayer::new(WasmLayerConfig::new()))
        .init();

    yew::Renderer::<App>::new().render();
}
