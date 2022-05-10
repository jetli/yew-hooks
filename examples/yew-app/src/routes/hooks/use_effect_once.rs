use gloo::dialogs::alert;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_effect_once` demo
#[function_component(MyComponent)]
fn my_component() -> Html {
    use_effect_once(|| {
        alert("Running effect once on mount");

        || alert("Running clean-up of effect on unmount")
    });

    html! {
        <>{ "My Component" }</>
    }
}

#[function_component(UseEffectOnce)]
pub fn effect_once() -> Html {
    let toggle = use_toggle("Mount", "Unmount");

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ *toggle }</button>
                    <p>
                        {
                            if *toggle == "Unmount" {
                                html! { <MyComponent /> }
                            } else {
                                html! {}
                            }
                        }
                    </p>
                </div>
            </header>
        </div>
    }
}
