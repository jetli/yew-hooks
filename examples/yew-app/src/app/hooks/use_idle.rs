use std::time::Duration;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_idle` demo
#[function_component]
pub fn UseIdle() -> Html {
    let idle = use_idle(Duration::from_secs(3)); // 5 seconds idle timeout

    let onreset = {
        let idle = idle.clone();
        Callback::from(move |_| idle.reset_idle())
    };

    let last_active = idle
        .last_active()
        .map(|ts| {
            let date = js_sys::Date::new(&js_sys::Number::from(ts));
            date.to_string()
                .as_string()
                .unwrap_or_else(|| "Invalid date".to_string())
        })
        .unwrap_or_else(|| "Never".to_string());

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onreset}>{ "Reset Idle Timer" }</Button>
                    <p>
                        <b>{ "User is idle: " }</b>
                        { if *idle { "Yes" } else { "No" } }
                    </p>
                    <p>
                        <b>{ "Last active: " }</b>
                        { last_active }
                    </p>
                    <p class="text-sm text-gray-600">
                        { "Try not to move your mouse or press any keys for 3 seconds." }
                    </p>
                </div>
            </header>
        </div>
    }
}
