use yew::prelude::*;

use yew_hooks::use_timeout;

/// `use_timeout` demo
#[function_component(UseTimeout)]
pub fn timeout() -> Html {
    let millis = use_state(|| 0);
    let state = use_state(|| 0);

    let onstart = {
        let millis = millis.clone();
        Callback::from(move |_| millis.set(2000))
    };
    let oncancel = {
        let millis = millis.clone();
        Callback::from(move |_| millis.set(0))
    };

    {
        let state = state.clone();
        use_timeout(
            move || {
                state.set(*state + 1);
            },
            *millis,
        );
    }

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onstart}>{ "Start timeout" }</button>
                    <button onclick={oncancel}>{ "Cancel timeout" }</button>
                    <p>
                        <b>{ "Timeout state: " }</b>
                        { *state }
                    </p>
                </div>
            </header>
        </div>
    }
}
