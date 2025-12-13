use gloo::dialogs::alert;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_unmount` demo
#[function_component]
fn MyComponent() -> Html {
    use_unmount(|| {
        alert("Unmount!");
    });

    html! {
        <>{ "My Component" }</>
    }
}

#[function_component]
pub fn UseUnmount() -> Html {
    let toggle = use_toggle("Unmount", "Mount");

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ html! { *toggle } }</Button>
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
