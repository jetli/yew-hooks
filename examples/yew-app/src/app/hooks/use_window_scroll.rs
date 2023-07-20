use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_window_scroll` demo
#[function_component]
pub fn UseWindowScroll() -> Html {
    let state = use_window_scroll();

    html! {
        <div class="container">
            <header class="mt-24 text-xl w-[3000px] h-[3000px]">
                <div class="space-x-4 space-y-4">
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
