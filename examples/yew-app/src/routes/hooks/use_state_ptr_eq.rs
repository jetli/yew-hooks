use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_state_ptr_eq` demo
#[function_component(UseStatePtrEq)]
pub fn state_ptr_eq() -> Html {
    let state = use_state_ptr_eq(|| "".to_string());
    let history = use_list(vec![]);

    let onclick = {
        let state = state.clone();
        // Set the state to the same string `Hello, world!` every time.
        Callback::from(move |_| state.set("Hello, world!".to_string()))
    };

    {
        let history = history.clone();
        // This effect will not run if use `use_state` or `use_state_eq`.
        use_effect_with_deps(
            move |message| {
                history.push((&**message).clone());

                || ()
            },
            state,
        );
    }

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onclick}>{ "Send Hello, world!" }</button>
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
