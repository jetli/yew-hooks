use gloo::dialogs::alert;

use yew::prelude::*;

use yew_hooks::{use_event, use_event_with_window};

/// `use_event` demo
#[function_component(UseEvent)]
pub fn event() -> Html {
    let button = use_node_ref();

    use_event(button.clone(), "click", move |_: MouseEvent| {
        alert("Clicked!");
    });

    use_event_with_window("keypress", move |e: KeyboardEvent| {
        alert(format!("{} is pressed!", e.key()).as_str());
    });

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button ref={button}>{ "Click me!" }</button>
                    <p>
                        <b>{ "or Press any key on your awesome keyboard. " }</b>
                    </p>
                </div>
            </header>
        </div>
    }
}
