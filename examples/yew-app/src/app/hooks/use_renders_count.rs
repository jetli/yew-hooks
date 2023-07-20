use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_renders_count` demo
#[function_component]
pub fn UseRendersCount() -> Html {
    let count = use_renders_count();
    let update = use_update();

    let onclick = Callback::from(move |_| {
        update();
    });

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ "Update" }</Button>
                    <p>
                        <b>{ "Count: " }</b>
                        { count }
                    </p>
                </div>
            </header>
        </div>
    }
}
