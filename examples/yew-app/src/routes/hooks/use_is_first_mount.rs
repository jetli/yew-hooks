use yew::prelude::*;

use yew_hooks::use_is_first_mount;

/// `use_is_first_mount` demo
#[function_component(UseIsFirstMount)]
pub fn is_first_mount() -> Html {
    let is_first = use_is_first_mount();

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ "Is first mount: " }</b>
                        { is_first }
                    </p>
                </div>
            </header>
        </div>
    }
}
