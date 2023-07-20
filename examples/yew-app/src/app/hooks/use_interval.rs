use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_interval` demo
#[function_component]
pub fn UseInterval() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onstart}>{ "Start interval" }</Button>
                    <Button onclick={oncancel}>{ "Cancel interval" }</Button>
                    <p>
                        <b>{ "Interval state: " }</b>
                        { *state }
                    </p>
                </div>
            </header>
        </div>
    }
}
