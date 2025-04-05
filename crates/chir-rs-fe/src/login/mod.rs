//! Login screen

use std::collections::HashSet;

use chir_rs_common::http_api::{
    auth::{LoginRequest, PasetoToken, Scope},
    errors::APIError,
};
use eyre::Context;
use gloo_net::http::Request;
use gloo_storage::{SessionStorage, Storage as _};
use tracing::info;
use unicode_normalization::UnicodeNormalization;
use wasm_bindgen::JsCast as _;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::{spawn, Route};

/// Login view
#[function_component]
pub fn Login() -> Html {
    let navigator = use_navigator();
    let username_handle = use_state(String::default);
    let username = username_handle.clone();
    let password_handle = use_state(String::default);
    let password = password_handle.clone();
    let on_username_input = Callback::from(move |e: InputEvent| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            username.set(input.value());
        }
    });
    let on_password_input = Callback::from(move |e: InputEvent| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            password.set(input.value());
        }
    });
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        let navigator = navigator.clone();
        e.prevent_default();
        let username = username_handle.trim();
        let password = password_handle.trim().nfkc().collect::<String>();
        let request = LoginRequest {
            username: username.to_string(),
            password,
            scopes: HashSet::from([Scope::Full]),
        };

        spawn(async move {
            let req_body = bincode::encode_to_vec(request, bincode::config::standard())
                .context("Encoding login request")?;
            let resp = Request::post("/.api/auth/login")
                .header("Content-Type", "application/x+bincode")
                .header("Accept", "application/x+bincode")
                .body(req_body)
                .context("Setting body for login request")?
                .send()
                .await
                .context("Sending request for login request")?;

            let status = resp.status();
            let resp_body = resp
                .binary()
                .await
                .context("Loading server response for login request")?;

            if status >= 400 {
                let error: APIError =
                    bincode::decode_from_slice(&resp_body, bincode::config::standard())
                        .context("Decoding error response")?
                        .0;
                Err(error).context("While logging in")?;
            } else {
                let token: PasetoToken =
                    bincode::decode_from_slice(&resp_body, bincode::config::standard())
                        .context("Decoding success response")?
                        .0;
                SessionStorage::set("api", token.to_paseto())?;
            }
            if let Some(navigator) = navigator {
                navigator.replace(&Route::Home);
            }
            Ok(())
        });

        info!("{e:?}");
    });
    html! {
        <form accept-charset="utf-8" onsubmit={onsubmit}>
            <label for="username">{"Username:"}</label>
            <input id="username" type="text" placeholder="https://lotte.chir.rs/" oninput={on_username_input} />
            <label for="password">{"Password:"}</label>
            <input id="password" type="password" placeholder="●●●●●●●●●●●●●●●●" oninput={on_password_input} />
            <input type="submit" value="Submit"/>
        </form>
    }
}
