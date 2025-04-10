use chir_rs_common::http_api::{errors::APIError, robots::RobotsRule};
use eyre::Context;
use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_custom_components::table::{
    types::{ColumnBuilder, TableData},
    Options, Table,
};
use yew_hooks::use_session_storage;
use yew_router::prelude::*;

use crate::{spawn, Route};

/// Render the robots list
#[function_component]
pub fn Robots() -> Html {
    let token = use_session_storage::<String>("api".to_string());
    let Some(token) = (*token).clone() else {
        return html! {
                <Redirect<Route> to={Route::Login} />
        };
    };

    let lines = use_state(Vec::new);
    let lines2 = lines.clone();
    let redraw = use_state(|| 0);
    let token2 = token.clone();
    use_effect_with(redraw.clone(), move |_| {
        let lines = lines2;
        spawn(async move {
            let resp = Request::get("/.api/robots")
                .header("Authorization", &format!("Bearer {token2}"))
                .header("Accept", "application/x+bincode")
                .send()
                .await
                .context("Loading robots.txt entries")?;
            let status = resp.status();
            let resp_body = resp
                .binary()
                .await
                .context("Loading server response for robots.txt request")?;
            if status >= 400 {
                let error: APIError =
                    bincode::decode_from_slice(&resp_body, bincode::config::standard())
                        .context("Decoding error response")?
                        .0;
                Err(error).context("While fetching robots.txt")?;
            } else {
                let entries: Vec<RobotsRule> =
                    bincode::decode_from_slice(&resp_body, bincode::config::standard())?.0;
                lines.set(entries);
            }
            Ok(())
        });
    });

    let columns = vec![
        ColumnBuilder::new("id")
            .orderable(true)
            .short_name("ID")
            .data_property("id")
            .build(),
        ColumnBuilder::new("user_agent")
            .orderable(true)
            .short_name("UA")
            .data_property("user_agent")
            .build(),
        ColumnBuilder::new("path")
            .orderable(true)
            .short_name("Path")
            .data_property("path")
            .build(),
        ColumnBuilder::new("allow")
            .orderable(true)
            .short_name("Allow")
            .data_property("allow")
            .build(),
        ColumnBuilder::new("delete")
            .short_name("Delete")
            .data_property("delete")
            .build(),
    ];

    // Table options
    let options = Options {
        unordered_class: Some("fa-sort".to_string()),
        ascending_class: Some("fa-sort-up".to_string()),
        descending_class: Some("fa-sort-down".to_string()),
        orderable_classes: vec!["mx-1".to_string(), "fa-solid".to_string()],
    };

    // Fill the table data structure with actual data
    let mut table_data = Vec::new();
    for data in lines.iter() {
        table_data.push(TableLine {
            id: data.id,
            user_agent: data.user_agent.clone(),
            path: data.path.clone(),
            allow: data.allow,
            redraw_handle: Some(redraw.clone()),
            token: token.clone(),
        });
    }

    let ua_handle = use_state(String::new);
    let ua = ua_handle.clone();
    let path_handle = use_state(String::new);
    let path = path_handle.clone();
    let allow_handle = use_state(|| false);
    let allow = allow_handle.clone();

    let on_ua_input = Callback::from(move |e: InputEvent| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            ua.set(input.value());
        }
    });

    let on_path_input = Callback::from(move |e: InputEvent| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            path.set(input.value());
        }
    });

    let on_allowed_change = Callback::from(move |e: Event| {
        let target = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            allow.set(input.checked());
        }
    });

    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let ua = (*ua_handle).clone();
        let path = (*path_handle).clone();
        let allow = *allow_handle;
        let request = RobotsRule {
            id: 0,
            user_agent: ua,
            path,
            allow,
        };
        let token = token.clone();
        let redraw = redraw.clone();
        spawn(async move {
            let req_body = bincode::encode_to_vec(request, bincode::config::standard())
                .context("Encoding login request")?;
            let resp = Request::post("/.api/robots")
                .header("Authorization", &format!("Bearer {token}"))
                .header("Content-Type", "application/x+bincode")
                .header("Accept", "application/x+bincode")
                .body(req_body)
                .context("Setting body for robots rule raccreation request")?
                .send()
                .await
                .context("Sending request for robots rule raccreation request")?;
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
            }
            redraw.set(redraw.wrapping_add(1));
            Ok(())
        });
    });

    html! {
        <>
            <form accept-charset="utf-8" onsubmit={onsubmit}>
                <label for="user-agent">{"User Agent:"}</label>
                <input id="user-agent" type="text" placeholder="ChatGPT-User" oninput={on_ua_input} />
                <label for="path">{"Path:"}</label>
                <input id="path" type="text" placeholder="/" oninput={on_path_input} />
                <label>
                    <input id="allowed" type="checkbox" />
                    {"Allowed to raccess resource"}
                </label>
                <input type="submit" value="Submit"/>
            </form>
            <Table<TableLine> options={options} classes={classes!("table", "table-hover")} columns={columns} data={table_data.clone()} orderable={true}/>
        </>
    }
}

#[derive(Clone, Serialize, Debug, Default)]
struct TableLine {
    pub id: i32,
    pub user_agent: String,
    pub path: String,
    pub allow: bool,
    #[serde(skip)]
    pub redraw_handle: Option<UseStateHandle<u32>>,
    pub token: String,
}

impl PartialEq<Self> for TableLine {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for TableLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl TableData for TableLine {
    fn get_field_as_html(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<Html> {
        match field_name {
            "id" => Ok(html! { self.id }),
            "user_agent" => Ok(html! { &self.user_agent }),
            "path" => Ok(html! { &self.path }),
            "allow" => Ok(html! { self.allow }),
            "delete" => {
                let token = self.token.clone();
                let redraw_handle = self.redraw_handle.clone();
                let id = self.id;
                Ok(html! {
                    <button onclick={move |_| {
                        let token = token.clone();
                        let redraw_handle = redraw_handle.clone();
                        spawn(async move {
                            let req = Request::delete(&format!("/.api/robots/{id}"))
                                .header("Authorization", &format!("Bearer {token}"))
                                .header("Accept", "application/x+bincode")
                                .send()
                                .await
                                .context("Deleting robots.txt entry")?;
                            let status = req.status();
                            if status >= 400 {
                                let  error: APIError =
                                    bincode::decode_from_slice(&req.binary().await?, bincode::config::standard())?.0;
                                Err(error).context("While deleting robots.txt entry")?;
                            } else if let Some(redraw_handle) = redraw_handle {
                                redraw_handle.set(redraw_handle.wrapping_add(1));
                            }
                            Ok(())
                        });
                    }}>{"Delete"}</button>
                })
            }
            _ => Ok(html! {}),
        }
    }

    fn get_field_as_value(
        &self,
        field_name: &str,
    ) -> yew_custom_components::table::error::Result<serde_value::Value> {
        match field_name {
            "id" => Ok(serde_value::Value::I32(self.id)),
            "user_agent" => Ok(serde_value::Value::String(self.user_agent.clone())),
            "path" => Ok(serde_value::Value::String(self.path.clone())),
            "allow" => Ok(serde_value::Value::Bool(self.allow)),
            _ => Ok(serde_value::Value::Unit),
        }
    }

    fn matches_search(&self, _: Option<String>) -> bool {
        true
    }
}
