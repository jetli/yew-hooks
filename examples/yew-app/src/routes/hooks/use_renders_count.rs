use yew::prelude::*;

use yew_hooks::{use_renders_count, use_update};

/// `use_renders_count` demo
#[function_component(UseRendersCount)]
pub fn renders_count() -> Html {
    let count = use_renders_count();
    let update = use_update();

    let onclick = Callback::from(move |_| {
        update();
    });

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ "Update" }</button>
                    <p>
                        <b>{ "Count: " }</b>
                        { count }
                    </p>
                </div>
            </header>
        </div>
    }
}
