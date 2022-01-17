use yew::prelude::*;

use yew_hooks::{use_title, use_toggle};

/// `use_title` demo
#[function_component(MyComponent)]
fn my_component() -> Html {
    use_title("This is an awesome title".to_string());

    html! {
        <>{ "My Component" }</>
    }
}

#[function_component(UseTitle)]
pub fn title() -> Html {
    let toggle = use_toggle("Mount to change title", "Unmount to restore title");

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
