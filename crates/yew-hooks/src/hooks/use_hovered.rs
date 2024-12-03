use yew::prelude::*;

use super::use_event;

/// A sensor hook that tracks whether HTML element is being hovered.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseHovered)]
/// fn hovered() -> Html {
///     let node = use_node_ref();
///     let state = use_hovered(node.clone());
///
///     html! {
///         <div ref={node}>
///             <b>{ " Hovered: " }</b>
///             { state }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_hovered(node: NodeRef) -> bool {
    let state = use_state_eq(|| false);

    {
        let state = state.clone();
        let node = node.clone();
        use_event(node, "mouseover", move |_: Event| {
            state.set(true);
        });
    }

    {
        let state = state.clone();
        use_event(node, "mouseout", move |_: Event| {
            state.set(false);
        });
    }

    *state
}
