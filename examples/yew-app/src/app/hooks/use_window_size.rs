use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_window_size` demo
#[function_component]
pub fn UseWindowSize() -> Html {
    let state = use_window_size();

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ " Width: " }</b>
                        { state.0 }
                        <b>{ " Height: " }</b>
                        { state.1 }
                    </p>
                    <p>
                        { "Try to resize the window of your browser." }
                    </p>
                </div>
            </header>
        </div>
    }
}
