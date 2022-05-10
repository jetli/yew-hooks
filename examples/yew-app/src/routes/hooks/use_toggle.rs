use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_toggle` demo
#[function_component(UseToggle)]
pub fn toggle() -> Html {
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
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onclick}>{ "Toggle" }</button>
                    <button onclick={onreset}>{ "Reset" }</button>
                    <button onclick={onset}>{ "Set to World" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *toggle }
                    </p>
                </div>
            </header>
        </div>
    }
}
