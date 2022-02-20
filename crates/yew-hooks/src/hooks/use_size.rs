use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Element;

use yew::prelude::*;

use super::use_raf_state;
use crate::web_sys_ext::{ResizeObserver, ResizeObserverEntry};

/// A sensor hook that tracks an HTML element's dimensions using the `ResizeObserver` API.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_size;
///
/// #[function_component(UseSize)]
/// fn size() -> Html {
///     let node =  use_node_ref();
///     let state = use_size(node.clone());
///     
///     html! {
///         <div ref={node}>
///             <b>{ " Width: " }</b>
///             { state.0 }
///             <b>{ " Height: " }</b>
///             { state.1 }
///         </div>
///     }
/// }
/// ```
pub fn use_size(node: NodeRef) -> (u32, u32) {
    let state = use_raf_state(|| (0, 0));

    {
        let state = state.clone();
        use_effect_with_deps(
            move |node| {
                let closure = Closure::wrap(Box::new(move |entries: Vec<ResizeObserverEntry>| {
                    for entry in entries.iter() {
                        let element = entry.target();
                        state.set((
                            element.client_width() as u32,
                            element.client_height() as u32,
                        ));
                    }
                })
                    as Box<dyn Fn(Vec<ResizeObserverEntry>)>);

                let observer = ResizeObserver::new(closure.as_ref().unchecked_ref()).unwrap_throw();
                // Forget the closure to keep it alive
                closure.forget();

                if let Some(element) = &node.cast::<Element>() {
                    observer.observe(element);
                }

                move || observer.disconnect()
            },
            node,
        );
    }

    *state
}
