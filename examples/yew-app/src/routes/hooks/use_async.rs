use serde::{Deserialize, Serialize};

use yew::prelude::*;

use yew_hooks::use_async;

/// `use_async` demo
#[function_component(UseAsync)]
pub fn async_demo() -> Html {
    let state = use_async(async move { fetch_repo("jetli/yew-hooks".to_string()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            state.run();
        })
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick} disabled={state.loading}>{ "Start loading repo: jetli/yew-hooks" }</button>
                    <p>
                        {
                            if state.loading {
                                html! { "Loading, wait a sec..." }
                            } else {
                                html! {}
                            }
                        }
                    </p>
                    {
                        if let Some(repo) = &state.data {
                            html! {
                                <>
                                    <p>{ "Repo name: " }<b>{ &repo.name }</b></p>
                                    <p>{ "Repo full name: " }<b>{ &repo.full_name }</b></p>
                                    <p>{ "Repo description: " }<b>{ &repo.description }</b></p>

                                    <p>{ "Owner name: " }<b>{ &repo.owner.login }</b></p>
                                    <p>{ "Owner avatar: " }<b><br/><img alt="avatar" src={repo.owner.avatar_url.clone()} /></b></p>
                                </>
                                }
                        } else {
                            html! {}
                        }
                    }
                    <p>
                        {
                            if let Some(error) = &state.error {
                                match error {
                                    Error::DeserializeError => html! { "DeserializeError" },
                                    Error::RequestError => html! { "RequestError" },
                                }
                            } else {
                                html! {}
                            }
                        }
                    </p>
                </div>
            </header>
        </div>
    }
}

/// You can use reqwest or other crates to fetch your api
async fn fetch_repo(repo: String) -> Result<Repo, Error> {
    let response = reqwest::get(format!("https://api.github.com/repos/{}", repo)).await;
    if let Ok(data) = response {
        if let Ok(repo) = data.json::<Repo>().await {
            Ok(repo)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct User {
    id: i32,
    login: String,
    avatar_url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Repo {
    id: i32,
    name: String,
    full_name: String,
    description: String,
    owner: User,
}

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
enum Error {
    RequestError,
    DeserializeError,
    // etc.
}
