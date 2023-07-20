use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_raf` demo
#[function_component]
pub fn UseRaf() -> Html {
    let elapsed = use_raf(5000, 1000);

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ "Elapsed: " }</b><br/>
                        { elapsed }
                    </p>
                </div>
            </header>
        </div>
    }
}
