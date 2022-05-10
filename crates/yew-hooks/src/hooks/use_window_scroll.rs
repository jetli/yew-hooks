use gloo::utils::window;
use yew::prelude::*;

use super::{use_event_with_window, use_mount, use_raf_state};

/// A sensor hook that tracks Window scroll position.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseWindowScroll)]
/// fn window_scroll() -> Html {
///     let state = use_window_scroll();
///     
///     html! {
///         <>
///             <b>{ " X: " }</b>
///             { state.0 }
///             <b>{ " Y: " }</b>
///             { state.1 }
///         </>
///     }
/// }
/// ```
pub fn use_window_scroll() -> (f64, f64) {
    let state = use_raf_state(|| {
        (
            window().page_x_offset().unwrap(),
            window().page_y_offset().unwrap(),
        )
    });

    {
        let state = state.clone();
        use_event_with_window("scroll", move |_: Event| {
            state.set((
                window().page_x_offset().unwrap(),
                window().page_y_offset().unwrap(),
            ));
        });
    }

    {
        let state = state.clone();
        use_mount(move || {
            state.set((
                window().page_x_offset().unwrap(),
                window().page_y_offset().unwrap(),
            ));
        });
    }

    *state
}
