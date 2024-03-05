use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{IntersectionObserver, IntersectionObserverEntry};
use yew::prelude::*;

use super::use_effect_once;

/// Check if an element is visible. Internally, it uses an [`IntersectionObserver`] to receive
/// notifications from the browser whenever the visibility state of the node changes.
///
/// Setting the sticky bit makes this hook disconnect the observer once the element is visible, and
/// keep the visibility set to `true`, even when it becomes invisible. This is often desired
/// for lazy-loading components.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component]
/// fn MyComponent() -> Html {
///     let node = use_node_ref();
///     let visible = use_visible(node.clone(), false);
///
///     html! {
///         <div ref={node}>
///             if visible {
///                 <p>{"I'm visible!"}</p>
///             } else {
///                 <p>{"I'm invisible!"}</p>
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_visible(node: NodeRef, sticky: bool) -> bool {
    // code adapted from:
    // https://stackoverflow.com/questions/1462138/event-listener-for-when-element-becomes-visible
    let visible = use_state_eq(|| false);
    let visible_clone = visible.clone();

    use_effect_once(move || {
        let closure = Closure::<dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver)>::new(
            move |entries: Vec<IntersectionObserverEntry>, observer: IntersectionObserver| {
                // determine if any part of this node is visible.
                let visible = entries.iter().any(|entry| entry.intersection_ratio() > 0.0);

                // if the visibility changed, update the state.
                visible_clone.set(visible);

                // if this is sticky and it is currently visible, disconnect the observer.
                if visible && sticky {
                    observer.disconnect();
                }
            },
        )
        .into_js_value();
        let observer = IntersectionObserver::new(closure.dyn_ref().unwrap()).unwrap();
        if let Some(node) = node.get() {
            observer.observe(node.dyn_ref().unwrap());
        }
        move || observer.disconnect()
    });

    *visible
}
