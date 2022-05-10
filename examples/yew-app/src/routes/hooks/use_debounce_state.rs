use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_debounce_state` demo
#[function_component(UseDebounceState)]
pub fn debounce_state() -> Html {
    let value = use_state(|| "".to_string());
    let debounced_value = use_debounce_state(|| "".to_string(), 2000);

    let oninput = {
        let value = value.clone();
        let debounced_value = debounced_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            // This will update state every time.
            value.set(input.value());
            // This will delay updating state.
            debounced_value.set(input.value());
        })
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <input type="text" value={(*value).clone()} placeholder="Debounced input" {oninput}/>
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
