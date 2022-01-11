use yew::prelude::*;

use yew_hooks::use_mount;

/// `use_mount` demo
#[function_component(UseMount)]
pub fn mount() -> Html {
    use_mount(|| {
        log::debug!("Running effect once on mount");
    });

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ "Please view console log" }</b>
                    </p>
                </div>
            </header>
        </div>
    }
}
