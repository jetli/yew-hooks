use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_is_first_mount` demo
#[function_component]
pub fn UseIsFirstMount() -> Html {
    let is_first = use_is_first_mount();

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ "Is first mount: " }</b>
                        { is_first }
                    </p>
                </div>
            </header>
        </div>
    }
}
