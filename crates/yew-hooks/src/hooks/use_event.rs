use std::borrow::Cow;
use std::ops::Deref;

use gloo::events::{EventListener, EventListenerOptions};
use gloo::utils::window;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::use_latest;

/// A hook that subscribes a callback to events.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseEvent)]
/// fn event() -> Html {
///     let button = use_node_ref();
///
///     use_event(button.clone(), "click", move |_: MouseEvent| {
///         debug!("Clicked!");
///     });
///     
///     html! {
///         <>
///             <button ref={button}>{ "Click me!" }</button>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_event<T, F, E>(node: NodeRef, event_type: T, callback: F)
where
    T: Into<Cow<'static, str>>,
    F: Fn(E) + 'static,
    E: From<JsValue>,
{
    let callback = use_latest(callback);

    use_effect_with_deps(
        move |(node, event_type)| {
            let window = window();
            let node = node.get();
            // If we cannot get the wrapped `Node`, then we use `Window` as the default target of the event.
            let target = node.as_deref().map_or(window.deref(), |t| t);

            // We should only set passive event listeners for `touchstart` and `touchmove`.
            // See here: https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#Improving_scrolling_performance_with_passive_listeners
            let listener = if event_type == "touchstart"
                || event_type == "touchmove"
                || event_type == "scroll"
            {
                Some(EventListener::new(
                    target,
                    event_type.clone(),
                    move |event| {
                        (*callback.current())(JsValue::from(event).into());
                    },
                ))
            } else {
                Some(EventListener::new_with_options(
                    target,
                    event_type.clone(),
                    EventListenerOptions::enable_prevent_default(),
                    move |event| {
                        (*callback.current())(JsValue::from(event).into());
                    },
                ))
            };

            move || drop(listener)
        },
        (node, event_type.into()),
    );
}

/// A hook that subscribes a callback to events only for window.
/// If you want to specify an event target, use [`use_event`].
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseEvent)]
/// fn event() -> Html {
///     use_event_with_window("keypress", move |e: KeyboardEvent| {
///         debug!("{} is pressed!", e.key());
///     });
///     
///     html! {
///         <>
///             { "Press any key on your awesome keyboard!" }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_event_with_window<T, F, E>(event_type: T, callback: F)
where
    T: Into<Cow<'static, str>>,
    F: Fn(E) + 'static,
    E: From<JsValue>,
{
    use_event(NodeRef::default(), event_type, callback);
}
