use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_counter` demo
#[function_component(UseCounter)]
pub fn counter() -> Html {
    let counter = use_counter(0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };
    let onincreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase_by(10))
    };
    let ondecreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease_by(10))
    };
    let onset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(100))
    };
    let onreset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.reset())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onincrease}>{ "Increase" }</button>
                    <button onclick={ondecrease}>{ "Decrease" }</button>
                    <button onclick={onincreaseby}>{ "Increase by 10" }</button>
                    <button onclick={ondecreaseby}>{ "Decrease by 10" }</button>
                    <button onclick={onset}>{ "Set to 100" }</button>
                    <button onclick={onreset}>{ "Reset" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *counter }
                    </p>
                </div>
            </header>
        </div>
    }
}
