use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_before_unload` demo
#[function_component]
pub fn UseBeforeUnload() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{if *dirty { "Disable" } else { "Enable" }}</Button>
                    <p>
                        <b>{if *dirty { "Try to reload or close tab." } else { "" }}</b>
                    </p>
                </div>
            </header>
        </div>
    }
}
