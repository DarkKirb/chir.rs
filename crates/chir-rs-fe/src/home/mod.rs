//! Homepage

use chir_rs_common::http_api::errors::APIError;
use eyre::Context;
use gloo_net::http::Request;
use gloo_storage::{SessionStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{spawn, Route};

/// Render the home page of the admin page
#[function_component]
pub fn HomePage() -> Html {
    let navigator = use_navigator();
    SessionStorage::get::<String>("api").map_or_else(
        |_| {
            html! {
                <Redirect<Route> to={Route::Login} />
            }
        },
        |token| {
            let logout = Callback::from(move |e: MouseEvent| {
                let navigator = navigator.clone();
                e.prevent_default();
                SessionStorage::delete("api");
                if let Some(navigator) = navigator {
                    navigator.replace(&Route::Login);
                }
                let token = token.clone();
                spawn(async move {
                    let resp = Request::delete("/.api/auth/login")
                        .header("Authorization", &format!("Bearer {token}"))
                        .send()
                        .await
                        .context("Sending request for logout")?;
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
                        Err(error).context("While logging out")?;
                    }
                    Ok(())
                });
            });
            html! {
                <div class="container">
                    <button onclick={logout}>{"Log out"}</button>
                    <ul>
                        <li>
                            <Link<Route> to={Route::Robots}>{"Robots"}</Link<Route>>
                        </li>
                    </ul>
                </div>
            }
        },
    )
}
