use gloo::dialogs::alert;

use yew::prelude::*;

use yew_hooks::{use_mount, use_toggle};

/// `use_mount` demo
#[function_component(MyComponent)]
fn my_component() -> Html {
    use_mount(|| {
        alert("Mount!");
    });

    html! {
        <>{ "My Component" }</>
    }
}

#[function_component(UseMount)]
pub fn mount() -> Html {
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
