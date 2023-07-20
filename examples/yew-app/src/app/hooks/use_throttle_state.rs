use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_throttle_state` demo
#[function_component]
pub fn UseThrottleState() -> Html {
    let state = use_throttle_state(|| 0, 2000);

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ "Click fast!" }</Button>
                    <p>
                        <b>{ "State: " }</b> {*state}
                    </p>
                </div>
            </header>
        </div>
    }
}
