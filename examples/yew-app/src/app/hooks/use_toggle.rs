use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_toggle` demo
#[function_component]
pub fn UseToggle() -> Html {
    let toggle = use_toggle("Hello", "World");

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
        Callback::from(move |_| toggle.set("World"))
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onclick}>{ "Toggle" }</Button>
                    <Button onclick={onreset}>{ "Reset" }</Button>
                    <Button onclick={onset}>{ "Set to World" }</Button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *toggle }
                    </p>
                </div>
            </header>
        </div>
    }
}
