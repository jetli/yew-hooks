# Yew Hooks

Hooks for [Yew](https://github.com/yewstack/yew), inspired by [streamich/react-use](https://github.com/streamich/react-use) and [alibaba/hooks](https://github.com/alibaba/hooks).

## Hooks

### State

- `use_toggle` - tracks state of counterparts.
- `use_bool_toggle` - tracks state of a boolean.
- `use_counter` -  tracks state of a number.
- `use_latest` - returns the latest immutable ref to state or props.
- `use_mut_latest` - returns the latest mutable ref to state or props.
- `use_previous` - returns the previous immutable ref to state or props.
- `use_list` - tracks state of a list.
- `use_map` - tracks state of a hash map.
- `use_set` - tracks state of a hash set.
- `use_queue` - tracks state of a queue.
- `use_raf_state` - creates `set` method which only updates after `requestAnimationFrame`.

### Side-effects

- `use_async` - resolves an async future, like fetch REST api.
- `use_title` - sets title of the page.
- `use_local_storage` - manages a value in `localStorage`.
- `use_session_storage` - manages a value in `sessionStorage`.
- `use_before_unload` - shows browser alert when user try to reload or close the page.

### Lifecycles

- `use_effect_once` - a modified use_effect hook that only runs once.
- `use_mount` - calls mount callbacks.
- `use_unmount` - calls unmount callbacks.
- `use_is_first_mount` - checks if current render is first.
- `use_is_mounted` - tracks if component is mounted.
- `use_event` - subscribes to events.

### Animations

- `use_timeout` - schedules a timeout to invoke callback.
- `use_interval` - schedules an interval to invoke callback.
- `use_update` - returns a callback, which re-renders component when called.
- `use_raf` - re-renders component on each `requestAnimationFrame`.

### Sensors

- `use_window_size` - tracks Window dimensions.

## Examples

### `use_counter` Demo

```rust
use yew::prelude::*;

use yew_hooks::use_counter;

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_counter(0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };
    let onincreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase_by(10))
    };
    let ondecreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease_by(10))
    };
    let onset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(100))
    };
    let onreset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.reset())
    };
    
    html! {
        <div>
            <button onclick={onincrease}>{ "Increase" }</button>
            <button onclick={ondecrease}>{ "Decrease" }</button>
            <button onclick={onincreaseby}>{ "Increase by 10" }</button>
            <button onclick={ondecreaseby}>{ "Decrease by 10" }</button>
            <button onclick={onset}>{ "Set to 100" }</button>
            <button onclick={onreset}>{ "Reset" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
```

### `use_async` Demo

```rust
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew::prelude::*;

use yew_hooks::use_async;

#[function_component(UseAsync)]
pub fn async_demo() -> Html {
    let state = use_async(async move { fetch_repo("jetli/yew-hooks".to_string()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            // You can trigger to run in callback or use_effect_with_deps.
            state.run();
        })
    };

    html! {
        <div>
            <button {onclick} disabled={state.loading}>{ "Start to load repo: jetli/yew-hooks" }</button>
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
    }
}

async fn fetch_repo(repo: String) -> Result<Repo, Error> {
    fetch::<Repo>(format!("https://api.github.com/repos/{}", repo)).await
}

/// You can use reqwest or other crates to fetch your api.
async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;
    if let Ok(data) = response {
        if let Ok(repo) = data.json::<T>().await {
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
```

## Demo

[Check out a live demo](https://jetli.github.io/yew-hooks/)
