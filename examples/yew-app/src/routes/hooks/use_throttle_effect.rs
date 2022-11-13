use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_throttle_effect` demo
#[function_component(UseThrottleEffect)]
pub fn throttle_effect() -> Html {
    let state = use_state(|| 0);
    let update = use_update();

    {
        let state = state.clone();
        use_throttle_effect(
            move || {
                state.set(*state + 1);
            },
            2000,
        );
    };

    let onclick = { Callback::from(move |_| update()) };

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
