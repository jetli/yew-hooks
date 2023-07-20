use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_title` demo
#[function_component]
fn MyComponent() -> Html {
    use_title("This is an awesome title".to_string());

    html! {
        <>{ "My Component" }</>
    }
}

#[function_component]
pub fn UseTitle() -> Html {
    let toggle = use_toggle("Mount to change title", "Unmount to restore title");

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ *toggle }</Button>
                    <p>
                        {
                            if *toggle == "Unmount to restore title" {
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
