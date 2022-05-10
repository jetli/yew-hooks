use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_default` demo
#[function_component(UseDefault)]
pub fn default() -> Html {
    let state = use_default(|| None, "Hello(default)".to_string());

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Some("World!".to_string()));
        })
    };

    let onclear = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(None);
        })
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ "Set to World!" }</button>
                    <button onclick={onclear}>{ "Clear" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { &*state }
                    </p>
                </div>
            </header>
        </div>
    }
}
