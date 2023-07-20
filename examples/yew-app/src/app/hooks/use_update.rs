use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_update` demo
#[function_component]
pub fn UseUpdate() -> Html {
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
                        <b>{ "Current time: " }</b>
                        { get_current_time() }
                    </p>
                </div>
            </header>
        </div>
    }
}

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}
