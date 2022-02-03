use yew::prelude::*;

use yew_hooks::{use_before_unload, use_bool_toggle};

/// `use_before_unload` demo
#[function_component(UseBeforeUnload)]
pub fn before_unload() -> Html {
    let dirty = use_bool_toggle(false);
    use_before_unload(
        *dirty,
        "You have unsaved changes, are you sure?".to_string(),
    );

    let onclick = {
        let dirty = dirty.clone();
        Callback::from(move |_| dirty.toggle())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{if *dirty { "Disable" } else { "Enable" }}</button>
                    <p>
                        <b>{if *dirty { "Try to reload or close tab." } else { "" }}</b>
                    </p>
                </div>
            </header>
        </div>
    }
}
