use gloo::dialogs::alert;

use yew::prelude::*;

use yew_hooks::{use_toggle, use_unmount};

/// `use_unmount` demo
#[function_component(MyComponent)]
fn my_component() -> Html {
    use_unmount(|| {
        alert("Unmount!");
    });

    html! {
        <>{ "My Component" }</>
    }
}

#[function_component(UseUnmount)]
pub fn unmount() -> Html {
    let toggle = use_toggle("Unmount", "Mount");

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
