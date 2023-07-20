use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_debounce` demo
#[function_component]
pub fn UseDebounce() -> Html {
    let status = use_state(|| "Typing stopped".to_string());
    let value = use_state(String::new);
    let debounced_value = use_state(String::new);

    let debounce = {
        let status = status.clone();
        let value = value.clone();
        let debounced_value = debounced_value.clone();
        use_debounce(
            move || {
                debounced_value.set((*value).clone());
                status.set("Typing stopped".to_string());
            },
            2000,
        )
    };

    let oninput = {
        let status = status.clone();
        let value = value.clone();
        let debounce = debounce.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            value.set(input.value());
            status.set("Waiting for typing to stop...".to_string());
            debounce.run();
        })
    };

    let onclick = { Callback::from(move |_| debounce.cancel()) };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <input class="flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" type="text" value={(*value).clone()} placeholder="Debounced input" {oninput}/>
                    <Button {onclick}>{ "Cancel debounce" }</Button>
                    <p>{&*status}</p>
                    <p>
                        <b>{ "Value: " }</b> {&*value}
                    </p>
                    <p>
                        <b>{ "Debounced value: " }</b> {&*debounced_value}
                    </p>
                </div>
            </header>
        </div>
    }
}
