use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_throttle` demo
#[function_component]
pub fn UseThrottle() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ "Click fast!" }</Button>
                    <Button onclick={oncancel}>{ "Cancel throttle" }</Button>
                    <p>
                        <b>{ "State: " }</b> {*state}
                    </p>
                </div>
            </header>
        </div>
    }
}
