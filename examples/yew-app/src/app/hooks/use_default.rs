use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_default` demo
#[function_component]
pub fn UseDefault() -> Html {
    let state = use_default(|| None, "Hello(default)".to_string());

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Some("World!".to_string()));
        })
    };

    let onclear = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(None);
        })
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ "Set to World!" }</Button>
                    <Button onclick={onclear}>{ "Clear" }</Button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { &*state }
                    </p>
                </div>
            </header>
        </div>
    }
}
