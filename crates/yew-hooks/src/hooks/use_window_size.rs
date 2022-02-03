use web_sys::{window, Window};

use yew::prelude::*;

use super::{use_event_with_window, use_raf_state};

/// A sensor hook that tracks dimensions of the browser window.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_window_size;
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

    *state
}
