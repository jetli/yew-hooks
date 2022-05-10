use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_interval` demo
#[function_component(UseInterval)]
pub fn interval() -> Html {
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
        use_interval(
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
                    <button onclick={onstart}>{ "Start interval" }</button>
                    <button onclick={oncancel}>{ "Cancel interval" }</button>
                    <p>
                        <b>{ "Interval state: " }</b>
                        { *state }
                    </p>
                </div>
            </header>
        </div>
    }
}
