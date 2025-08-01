use serde::{de::DeserializeOwned, Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_async` demo
#[function_component]
pub fn UseAsync() -> Html {
    let repo = use_state(|| "jetli/yew-hooks".to_string());
    // Demo #1, manually call `run` to load data.
    let state = {
        let repo = repo.clone();
        use_async(async move { fetch_repo((*repo).clone()).await })
    };

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            // You can manually trigger to run in callback or use_effect.
            state.run();
        })
    };

    let oninput = {
        let repo = repo.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            repo.set(input.value());
        })
    };

    // Demo #2, automatically load data when mount
    let _ = {
        let repo = repo.clone();
        use_async_with_options(
            async move { fetch_repo((*repo).clone()).await },
            // This will load data automatically when mount.
            UseAsyncOptions::enable_auto(),
        )
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <input class="flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" placeholder="Repo" value={(*repo).clone()} {oninput}/>
                    <Button {onclick} disabled={state.loading}>{ "Start to load repo" }</Button>
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
                        state.data.as_ref().map_or_else(|| html! {}, |repo| html! {
                            <>
                                <p>{ "Repo name: " }<b>{ &repo.name }</b></p>
                                <p>{ "Repo full name: " }<b>{ &repo.full_name }</b></p>
                                <p>{ "Repo description: " }<b>{ &repo.description }</b></p>

                                <p>{ "Owner name: " }<b>{ &repo.owner.login }</b></p>
                                <p>{ "Owner avatar: " }<b><br/><img class="mx-auto" alt="avatar" src={repo.owner.avatar_url.clone()} /></b></p>
                            </>
                            })
                    }
                    <p>
                        {
                            state.error.as_ref().map_or_else(|| html! {}, |error| match error {
                                Error::DeserializeError => html! { "DeserializeError" },
                                Error::RequestError => html! { "RequestError" },
                            })
                        }
                    </p>
                </div>
            </header>
        </div>
    }
}

async fn fetch_repo(repo: String) -> Result<Repo, Error> {
    fetch::<Repo>(format!("https://api.github.com/repos/{repo}")).await
}

/// You can use reqwest or other crates to fetch your api.
async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        (data.json::<T>().await).map_or(Err(Error::DeserializeError), |repo| Ok(repo))
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
