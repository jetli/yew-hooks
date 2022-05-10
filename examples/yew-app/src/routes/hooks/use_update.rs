use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_update` demo
#[function_component(UseUpdate)]
pub fn update() -> Html {
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
