use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_debounce_effect` demo
#[function_component(UseDebounceEffect)]
pub fn debounce_effect() -> Html {
    let status = use_state(|| "Typing stopped".to_string());
    let value = use_state(|| "".to_string());
    let debounced_value = use_state(|| "".to_string());

    {
        let status = status.clone();
        let value = value.clone();
        let debounced_value = debounced_value.clone();
        use_debounce_effect(
            move || {
                // This will delay updating state.
                debounced_value.set((*value).clone());
                status.set("Typing stopped".to_string());
            },
            2000,
        );
    }

    let oninput = {
        let status = status.clone();
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            // This will update state every time.
            value.set(input.value());
            status.set("Waiting for typing to stop...".to_string());
        })
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <input type="text" value={(*value).clone()} placeholder="Debounced input" {oninput}/>
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
