use yew::prelude::*;

use yew_hooks::use_unmount;

/// `use_unmount` demo
#[function_component(UseUnmount)]
pub fn unmount() -> Html {
    use_unmount(|| {
        log::debug!("Running clean-up of effect on unmount");
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
