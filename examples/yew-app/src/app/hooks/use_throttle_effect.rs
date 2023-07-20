use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_throttle_effect` demo
#[function_component]
pub fn UseThrottleEffect() -> Html {
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
