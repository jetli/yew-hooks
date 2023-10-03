use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_state_ptr_eq` demo
#[function_component]
pub fn UseStatePtrEq() -> Html {
    let state = use_state_ptr_eq(String::new);
    let history = use_list(vec![]);

    let onclick = {
        let state = state.clone();
        // Set the state to the same string `Hello, world!` every time.
        Callback::from(move |_| state.set("Hello, world!".to_string()))
    };

    {
        let history = history.clone();
        // This effect will not run if use `use_state` or `use_state_eq`.
        use_effect_with(state, move |message| {
            history.push((**message).clone());

            || ()
        });
    }

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onclick}>{ "Send Hello, world!" }</Button>
                    <p>
                        <b>{ "Message history: " }</b>
                    </p>
                    {
                        for history.current().iter().map(|message| {
                            html! {
                                <p><b>{ message }</b></p>
                            }
                        })
                    }
                </div>
            </header>
        </div>
    }
}
