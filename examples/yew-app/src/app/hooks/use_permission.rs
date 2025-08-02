use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_permission` demo
#[function_component]
pub fn UsePermission() -> Html {
    let state = use_permission("notifications".to_owned());

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ "state: " }</b>
                        { format!("{state:?}") }
                    </p>
                </div>
            </header>
        </div>
    }
}
