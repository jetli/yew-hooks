use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_window_size` demo
#[function_component(UseWindowSize)]
pub fn window_size() -> Html {
    let state = use_window_size();

    html! {
        <div class="app">
            <header class="app-header">
                <div>
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
