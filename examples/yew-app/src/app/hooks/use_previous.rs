use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_previous` demo
#[function_component]
pub fn UsePrevious() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onincrease}>{ "Increase" }</Button>
                    <Button onclick={ondecrease}>{ "Decrease" }</Button>
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
