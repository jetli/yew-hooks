use web_sys::{window, Window};
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_raf_state` demo
#[function_component(UseRafState)]
pub fn raf_state() -> Html {
    let state = use_raf_state(|| {
        (
            window().unwrap().inner_width().unwrap().as_f64().unwrap(),
            window().unwrap().inner_height().unwrap().as_f64().unwrap(),
        )
    });

    {
        let state = state.clone();
        use_event_with_window("resize", move |e: Event| {
            let window: Window = e.target_unchecked_into();
            state.set((
                window.inner_width().unwrap().as_f64().unwrap(),
                window.inner_height().unwrap().as_f64().unwrap(),
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
