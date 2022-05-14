use yew::prelude::*;

use super::{use_debounce, use_event};

/// A sensor hook that tracks whether HTML element is scrolling.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseScrolling)]
/// fn scrolling() -> Html {
///     let node = use_node_ref();
///     let state = use_scrolling(node.clone());
///     
///     html! {
///         <div ref={node}>
///             <b>{ " Scrolling: " }</b>
///             { state }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_scrolling(node: NodeRef) -> bool {
    let state = use_state_eq(|| false);

    let debounce = {
        let state = state.clone();
        use_debounce(
            move || {
                state.set(false);
            },
            150,
        )
    };

    {
        let state = state.clone();
        use_event(node, "scroll", move |_: Event| {
            state.set(true);
            debounce.run();
        });
    }

    *state
}
