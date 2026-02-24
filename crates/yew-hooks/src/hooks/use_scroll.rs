use web_sys::Element;
use yew::prelude::*;

use super::{use_event, use_mount};

/// A sensor hook that tracks an HTML element's scroll position.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseScroll)]
/// fn scroll() -> Html {
///     let node = use_node_ref();
///     let state = use_scroll(node.clone());
///     
///     html! {
///         <div ref={node}>
///             <b>{ " X: " }</b>
///             { state.0 }
///             <b>{ " Y: " }</b>
///             { state.1 }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_scroll(node: NodeRef) -> (i32, i32) {
    let state = use_state(|| (0, 0));

    {
        let state = state.clone();
        let node = node.clone();
        use_event(node, "scroll", move |e: Event| {
            let element: Element = e.target_unchecked_into();
            #[allow(clippy::unnecessary_cast)]
            state.set((element.scroll_left() as i32, element.scroll_top() as i32));
        });
    }

    {
        let state = state.clone();
        use_mount(move || {
            if let Some(element) = node.cast::<Element>() {
                #[allow(clippy::unnecessary_cast)]
                state.set((element.scroll_left() as i32, element.scroll_top() as i32));
            }
        });
    }

    *state
}
