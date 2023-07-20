use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_counter` demo
#[function_component]
pub fn UseCounter() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onincrease}>{ "Increase" }</Button>
                    <Button onclick={ondecrease}>{ "Decrease" }</Button>
                    <Button onclick={onincreaseby}>{ "Increase by 10" }</Button>
                    <Button onclick={ondecreaseby}>{ "Decrease by 10" }</Button>
                    <Button onclick={onset}>{ "Set to 100" }</Button>
                    <Button onclick={onreset}>{ "Reset" }</Button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *counter }
                    </p>
                </div>
            </header>
        </div>
    }
}
