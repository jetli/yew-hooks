use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Element;
use yew::prelude::*;

use super::use_raf_state;
use crate::web_sys_ext::{ResizeObserver, ResizeObserverEntry};

#[derive(PartialEq, Default, Clone)]
pub struct UseMeasureState {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

/// A sensor hook that tracks an HTML element's dimensions using the `ResizeObserver` API.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseMeasure)]
/// fn measure() -> Html {
///     let node =  use_node_ref();
///     let state = use_measure(node.clone());
///     
///     html! {
///         <div ref={node}>
///             <b>{ " X: " }</b>
///             { state.x }
///             <b>{ " Y: " }</b>
///             { state.y }
///             <b>{ " Width: " }</b>
///             { state.width }
///             <b>{ " Height: " }</b>
///             { state.height }
///             <b>{ " Top: " }</b>
///             { state.top }
///             <b>{ " Left: " }</b>
///             { state.left }
///             <b>{ " Bottom: " }</b>
///             { state.bottom }
///             <b>{ " Right: " }</b>
///             { state.right }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_measure(node: NodeRef) -> UseMeasureState {
    let state = use_raf_state(UseMeasureState::default);

    {
        let state = state.clone();
        use_effect_with_deps(
            move |node| {
                let closure = Closure::wrap(Box::new(move |entries: Vec<ResizeObserverEntry>| {
                    for entry in &entries {
                        let rect = entry.content_rect();
                        state.set(UseMeasureState {
                            x: rect.x(),
                            y: rect.y(),
                            width: rect.width(),
                            height: rect.height(),
                            top: rect.top(),
                            left: rect.left(),
                            bottom: rect.bottom(),
                            right: rect.right(),
                        });
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

    (*state).clone()
}
