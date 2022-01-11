use yew::prelude::*;

use yew_hooks::use_effect_once;

/// `use_effect_once` demo
#[function_component(UseEffectOnce)]
pub fn effect_once() -> Html {
    use_effect_once(|| {
        log::debug!("Running effect once on mount");

        || log::debug!("Running clean-up of effect on unmount")
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
