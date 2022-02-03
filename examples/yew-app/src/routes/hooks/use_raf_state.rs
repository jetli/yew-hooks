use web_sys::Window;

use yew::prelude::*;

use yew_hooks::{use_event_with_window, use_raf_state};

/// `use_raf_state` demo
#[function_component(UseRafState)]
pub fn raf_state() -> Html {
    let state = use_raf_state(|| (0f64, 0f64));

    {
        let state = state.clone();
        use_event_with_window("resize", move |e: Event| {
            let element: Window = e.target_unchecked_into();
            state.set((
                element.inner_width().unwrap().as_f64().unwrap(),
                element.inner_height().unwrap().as_f64().unwrap(),
            ));
        });
    }

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
