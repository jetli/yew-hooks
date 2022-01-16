use gloo::timers::future::sleep;
use std::time::Duration;

use yew::prelude::*;

use yew_hooks::use_async;

/// `use_async` demo
#[function_component(UseAsync)]
pub fn async_demo() -> Html {
    let state = use_async(async move { fetch("/api/user/123".to_string()).await });

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
                    <button {onclick}>{ "Start loading" }</button>
                    <p>
                        {
                            if state.loading {
                                html! { "Loading" }
                            } else {
                                html! {}
                            }
                        }
                    </p>
                    <p>
                        {
                            if let Some(data) = &state.data {
                                html! { data }
                            } else {
                                html! {}
                            }
                        }
                    </p>
                    <p>
                        {
                            if let Some(error) = &state.error {
                                html! { error }
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

async fn fetch(_url: String) -> Result<String, String> {
    // You can use reqwest or other crates to fetch your api
    sleep(Duration::from_secs(3)).await;
    Ok(String::from("Jet Li"))
}
