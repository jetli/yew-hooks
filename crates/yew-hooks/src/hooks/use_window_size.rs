use gloo::utils::window;
use yew::prelude::*;

use super::{use_event_with_window, use_mount, use_raf_state};

/// A sensor hook that tracks dimensions of the browser window.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseWindowSize)]
/// fn window_size() -> Html {
///     let state = use_window_size();
///     
///     html! {
///         <>
///             <b>{ " Width: " }</b>
///             { state.0 }
///             <b>{ " Height: " }</b>
///             { state.1 }
///         </>
///     }
/// }
/// ```
pub fn use_window_size() -> (f64, f64) {
    let state = use_raf_state(|| {
        (
            window().inner_width().unwrap().as_f64().unwrap(),
            window().inner_height().unwrap().as_f64().unwrap(),
        )
    });

    {
        let state = state.clone();
        use_event_with_window("resize", move |_: Event| {
            state.set((
                window().inner_width().unwrap().as_f64().unwrap(),
                window().inner_height().unwrap().as_f64().unwrap(),
            ));
        });
    }

    {
        let state = state.clone();
        use_mount(move || {
            state.set((
                window().inner_width().unwrap().as_f64().unwrap(),
                window().inner_height().unwrap().as_f64().unwrap(),
            ));
        });
    }

    *state
}
