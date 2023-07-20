use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_debounce_state` demo
#[function_component]
pub fn UseDebounceState() -> Html {
    let value = use_state(String::new);
    let debounced_value = use_debounce_state(String::new, 2000);

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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <input class="flex h-10 w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50" type="text" value={(*value).clone()} placeholder="Debounced input" {oninput}/>
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
