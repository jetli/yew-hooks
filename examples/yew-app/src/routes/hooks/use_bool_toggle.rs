use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_bool_toggle` demo
#[function_component(UseBoolToggle)]
pub fn bool_toggle() -> Html {
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
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ "Toggle" }</button>
                    <button onclick={onreset}>{ "Reset" }</button>
                    <button onclick={onset}>{ "Set to false" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *toggle }
                    </p>
                </div>
            </header>
        </div>
    }
}
