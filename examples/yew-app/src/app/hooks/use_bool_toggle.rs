use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_bool_toggle` demo
#[function_component]
pub fn UseBoolToggle() -> Html {
    let toggle = use_bool_toggle(true);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };
    let onreset = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.reset())
    };
    let onset = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.set(false))
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ "Toggle" }</Button>
                    <Button onclick={onreset}>{ "Reset" }</Button>
                    <Button onclick={onset}>{ "Set to false" }</Button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *toggle }
                    </p>
                </div>
            </header>
        </div>
    }
}
