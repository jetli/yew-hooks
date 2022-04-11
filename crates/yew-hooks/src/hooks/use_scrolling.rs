use gloo::timers::callback::Timeout;
use yew::prelude::*;

use super::{use_event, use_unmount};

/// A sensor hook that tracks whether HTML element is scrolling.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_scrolling;
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
pub fn use_scrolling(node: NodeRef) -> bool {
    let state = use_state(|| false);
    let timer = use_mut_ref(|| None);

    {
        let state = state.clone();
        let timer = timer.clone();
        use_event(node, "scroll", move |_: Event| {
            state.set(true);
            let state = state.clone();
            *timer.borrow_mut() = Some(Timeout::new(150, move || {
                state.set(false);
            }));
        });
    }

    use_unmount(move || {
        *timer.borrow_mut() = None;
    });

    *state
}
