//! Homepage

use gloo_storage::{SessionStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Render the home page of the admin page
pub fn home_page() -> Html {
    SessionStorage::get::<String>("api").map_or_else(
        |_| {
            html! {
                <Redirect<Route> to={Route::Login} />
            }
        },
        |api_key| html! {<p>{api_key}</p>},
    )
}
