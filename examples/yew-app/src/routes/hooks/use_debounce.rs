use web_sys::HtmlInputElement;

use yew::prelude::*;

use yew_hooks::use_debounce;

/// `use_debounce` demo
#[function_component(UseDebounce)]
pub fn debounce() -> Html {
    let status = use_state(|| "Typing stopped".to_string());
    let value = use_state(|| "".to_string());
    let debounced_value = use_state(|| "".to_string());

    let debounce = {
        let value = value.clone();
        let value2 = value.clone();
        let status = status.clone();
        let debounced_value = debounced_value.clone();
        use_debounce(
            move || {
                debounced_value.set((*value2).clone());
                status.set("Typing stopped".to_string());
            },
            2000,
            value,
        )
    };

    let oninput = {
        let status = status.clone();
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            value.set(input.value());
            status.set("Waiting for typing to stop...".to_string());
        })
    };

    let onclick = { Callback::from(move |_| debounce.cancel()) };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <input type="text" value={(*value).clone()} placeholder="Debounced input" {oninput}/>
                    <button {onclick}>{ "Cancel debounce" }</button>
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
