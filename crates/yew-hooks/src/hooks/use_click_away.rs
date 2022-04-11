use std::rc::Rc;

use web_sys::Node;
use yew::{prelude::*, TargetCast};

use super::{use_event_with_window, use_latest};

/// A hook that triggers a callback when user clicks outside the target element.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::use_click_away;
///
/// #[function_component(UseClickAway)]
/// fn click_away() -> Html {
///     let node = use_node_ref();
///
///     use_click_away(node.clone(), move |_: Event| {
///         debug!("Clicked outside!");
///     });
///     
///     html! {
///         <div ref={node}>
///             { "Try to click outside of this area." }
///         </div>
///     }
/// }
/// ```
pub fn use_click_away<F>(node: NodeRef, callback: F)
where
    F: Fn(Event) + 'static,
{
    let callback_ref = use_latest(callback);

    let handler = Rc::new(move |e: Event| {
        if let Some(node) = &node.get() {
            if let Some(target_node) = e.target_dyn_into::<Node>() {
                if !node.contains(Some(&target_node)) {
                    (*callback_ref.current())(e);
                }
            }
        }
    });

    {
        let handler = handler.clone();
        use_event_with_window("mousedown", move |e: Event| {
            handler(e);
        });
    }

    use_event_with_window("touchstart", move |e: Event| {
        handler(e);
    });
}
