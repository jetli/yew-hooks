use yew::prelude::*;

use yew_hooks::use_previous;

/// `use_previous` demo
#[function_component(UsePrevious)]
pub fn previous() -> Html {
    let state = use_state(|| 0);
    let previous_state = use_previous(state.clone());

    let onincrease = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };
    let ondecrease = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onincrease}>{ "Increase" }</button>
                    <button onclick={ondecrease}>{ "Decrease" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *state }
                    </p>
                    <p>
                        <b>{ "Previous value: " }</b>
                        { **previous_state }
                    </p>
                </div>
            </header>
        </div>
    }
}
