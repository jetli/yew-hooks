use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_throttle_state` demo
#[function_component(UseThrottleState)]
pub fn throttle_state() -> Html {
    let state = use_throttle_state(|| 0, 2000);

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ "Click fast!" }</button>
                    <p>
                        <b>{ "State: " }</b> {*state}
                    </p>
                </div>
            </header>
        </div>
    }
}
