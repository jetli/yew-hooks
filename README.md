<!-- markdownlint-disable MD033 -->

<h1 align="center">Yew Hooks</h1>

<div align="center">
    <!-- Version -->
    <a href="https://crates.io/crates/yew-hooks">
        <img src="https://img.shields.io/crates/v/yew-hooks.svg"
            alt="crates.io Version" />
    </a>
    <!-- Downloads -->
    <a href="https://crates.io/crates/yew-hooks">
        <img src="https://img.shields.io/crates/d/yew-hooks.svg"
            alt="crates.io Downloads" />
    </a>
    <!-- Docs -->
    <a href="https://docs.rs/yew-hooks">
        <img src="https://img.shields.io/badge/docs-latest-blue.svg"
            alt="docs.rs Docs" />
    </a>
    <!-- CI -->
    <a href="https://github.com/jetli/yew-hooks/actions">
        <img src="https://github.com/jetli/yew-hooks/actions/workflows/rust.yml/badge.svg"
            alt="Github actions CI status" />
    </a>
</div>

<div align="center">
    <h3>
        <a href="https://jetli.github.io/yew-hooks/"> Demos </a>
        <span> | </span>
        <a href="https://github.com/jetli/yew-hooks/tree/main/examples/yew-app"> Examples </a>
        <span> | </span>
        <a href="https://docs.rs/yew-hooks"> Docs </a>
    </h3>
</div>

<br/>

Hooks for [Yew](https://github.com/yewstack/yew), inspired by [streamich/react-use](https://github.com/streamich/react-use), [alibaba/hooks](https://github.com/alibaba/hooks) and [vueuse/vueuse](https://github.com/vueuse/vueuse).

```rust
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
    
    html! {
        <>
            <button onclick={onincrease}>{ "Increase" }</button>
            <button onclick={ondecrease}>{ "Decrease" }</button>
            <b>{ "Current value: " }</b>
            { *counter }
        </>
    }
}
```

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
- `use_state_ptr_eq` - similar to `use_state_eq`, but checks if the two `Rc`s of values point to the same allocation.
- `use_renders_count` - counts component renders.
- `use_default` - returns the default value when state is None.
- `use_debounce_state` - debounces state.

### Side-effects

- `use_async` - resolves an `async` future, e.g. fetching REST api.
- `use_web_socket` - communicates with `WebSocket`.
- `use_title` - sets title of the page.
- `use_local_storage` - manages a value in `localStorage`.
- `use_session_storage` - manages a value in `sessionStorage`.
- `use_before_unload` - shows browser alert when user try to reload or close the page.
- `use_debounce` - debounces a function.

### Lifecycles

- `use_effect_once` - a modified use_effect hook that only runs once.
- `use_effect_update` - runs an effect only on updates.
- `use_mount` - calls mount callbacks.
- `use_unmount` - calls unmount callbacks.
- `use_is_first_mount` - checks if current render is first.
- `use_is_mounted` - tracks if component is mounted.
- `use_event` - subscribes to events.
- `use_logger` - logs in console as component goes through life cycles.

### Animations

- `use_timeout` - schedules a timeout to invoke callback.
- `use_interval` - schedules an interval to invoke callback.
- `use_update` - returns a callback, which re-renders component when called.
- `use_raf` - re-renders component on each `requestAnimationFrame`.

### Sensors

- `use_window_size` - tracks Window dimensions.
- `use_window_scroll` - tracks Window scroll position.
- `use_scroll` - tracks an HTML element's scroll position.
- `use_scrolling` - tracks whether HTML element is scrolling.
- `use_location` - tracks brower's location value.
- `use_hash` - tracks brower's location hash value.
- `use_search_param` - tracks brower's location search param value.
- `use_size` - tracks an HTML element's dimensions using the `ResizeObserver` API.
- `use_measure` - tracks an HTML element's dimensions using the `ResizeObserver` API.
- `use_geolocation` - tracks user's geographic location.
- `use_swipe` - detects swipe based on TouchEvent.

### UI

- `use_click_away` - triggers a callback when user clicks outside the target element.
- `use_drag` - tracks file, link and copy-paste drags, used along with `use_drop` hook.
- `use_drop` - tracks file, link and copy-paste drops.
- `use_media` - plays video or audio and exposes its controls.

## Examples

### `use_counter` demo

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

### `use_async` demo

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
            // You can trigger to run in callback or use_effect.
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

### `use_web_socket` demo

```rust
use yew::prelude::*;

use yew_hooks::{use_list, use_web_socket, UseWebSocketReadyState};

#[function_component(UseWebSocket)]
pub fn web_socket() -> Html {
    let history = use_list(vec![]);
    let ws = use_web_socket("wss://echo.websocket.events/".to_string());

    let onclick = {
        let ws = ws.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, world!".to_string();
            ws.send(message.clone());
            history.push(format!("[send]: {}", message));
        })
    };

    {
        let history = history.clone();
        let ws = ws.clone();
        use_effect_with_deps(
            move |message| {
                if let Some(message) = &**message {
                    history.push(format!("[recv]: {}", message.clone()));
                }
                || ()
            },
            ws.message,
        );
    }

    html! {
        <div>
            <p>
                <button {onclick} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send" }</button>
            </p>
            <p>
                <b>{ "Message history: " }</b>
            </p>
            {
                for history.current().iter().map(|message| {
                    html! {
                        <p>{ message }</p>
                    }
                })
            }
        </div>
    }
}
```

## Demo

[Check out a live demo](https://jetli.github.io/yew-hooks/)

## Contribute

Feel free to take a look at the current issues in this repo for anything that currently needs to be worked on.

You are also welcome to open a PR or a new issue if you see something is missing or could be improved upon.

## License

Apache-2.0/MIT
