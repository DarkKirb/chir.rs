//! Login screen

use tracing::info;
use yew::prelude::*;

/// Login view
pub fn login() -> Html {
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        info!("{e:?}");
    });
    html! {
        <form accept-charset="utf-8" onsubmit={onsubmit}>
            <label for="username">{"Username:"}</label>
            <input id="username" type="text" placeholder="https://lotte.chir.rs/" />
            <label for="password">{"Password:"}</label>
            <input id="password" type="password" placeholder="●●●●●●●●●●●●●●●●" />
            <input type="submit" value="Submit"/>
        </form>
    }
}
