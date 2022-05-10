use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_throttle` demo
#[function_component(UseThrottle)]
pub fn throttle() -> Html {
    let state = use_state(|| 0);

    let throttle = {
        let state = state.clone();
        use_throttle(
            move || {
                state.set(*state + 1);
            },
            2000,
        )
    };

    let onclick = {
        let throttle = throttle.clone();
        Callback::from(move |_| throttle.run())
    };

    let oncancel = { Callback::from(move |_| throttle.cancel()) };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ "Click fast!" }</button>
                    <button onclick={oncancel}>{ "Cancel throttle" }</button>
                    <p>
                        <b>{ "State: " }</b> {*state}
                    </p>
                </div>
            </header>
        </div>
    }
}
