use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_timeout` demo
#[function_component]
pub fn UseTimeout() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onreset}>{ "Reset timeout" }</Button>
                    <Button onclick={oncancel}>{ "Cancel timeout" }</Button>
                    <Button onclick={onincrease}>{ "Increase timeout millis" }</Button>
                    <p>
                        <b>{ "Timeout state: " }</b>
                        { *state }
                    </p>
                </div>
            </header>
        </div>
    }
}
