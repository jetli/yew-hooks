use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_timeout` demo
#[function_component(UseTimeout)]
pub fn timeout() -> Html {
    let state = use_state(|| 0);
    let millis = use_state(|| 2000);

    let timeout = {
        let state = state.clone();
        let millis = millis.clone();
        use_timeout(
            move || {
                state.set(*state + 1);
            },
            *millis,
        )
    };

    let onreset = {
        let timeout = timeout.clone();
        Callback::from(move |_| timeout.reset())
    };

    let oncancel = { Callback::from(move |_| timeout.cancel()) };

    let onincrease = { Callback::from(move |_| millis.set(*millis + 1000)) };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onreset}>{ "Reset timeout" }</button>
                    <button onclick={oncancel}>{ "Cancel timeout" }</button>
                    <button onclick={onincrease}>{ "Increase timeout millis" }</button>
                    <p>
                        <b>{ "Timeout state: " }</b>
                        { *state }
                    </p>
                </div>
            </header>
        </div>
    }
}
