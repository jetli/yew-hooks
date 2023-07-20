use gloo::dialogs::alert;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_event` demo
#[function_component]
pub fn UseEvent() -> Html {
    let button = use_node_ref();

    use_event(button.clone(), "click", move |_: MouseEvent| {
        alert("Clicked!");
    });

    use_event_with_window("keypress", move |e: KeyboardEvent| {
        alert(format!("{} is pressed!", e.key()).as_str());
    });

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button button_ref={button}>{ "Click me!" }</Button>
                    <p>
                        <b>{ "or Press any key on your awesome keyboard. " }</b>
                    </p>
                </div>
            </header>
        </div>
    }
}
