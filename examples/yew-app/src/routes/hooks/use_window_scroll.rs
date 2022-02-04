use yew::prelude::*;

use yew_hooks::use_window_scroll;

/// `use_window_size` demo
#[function_component(UseWindowScroll)]
pub fn window_scroll() -> Html {
    let state = use_window_scroll();

    html! {
        <div class="app" style="width: 3000px; height: 3000px;">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ " X: " }</b>
                        { state.0 }
                        <b>{ " Y: " }</b>
                        { state.1 }
                    </p>
                    <p>
                        { "Try to scroll this page vertically or horizontally." }
                    </p>
                </div>
            </header>
        </div>
    }
}
