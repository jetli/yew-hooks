use gloo::utils::window;
use wasm_bindgen::{prelude::*, JsCast};
use yew::prelude::*;

use super::use_mount;

/// A hook that tracks if a CSS media query matches.
///
/// This hook returns a boolean value indicating whether the given media query
/// currently matches. It updates automatically when the window is resized or
/// when the media query match status changes.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseMediaQuery)]
/// fn media_query() -> Html {
///     let is_small_screen = use_media_query("(max-width: 600px)");
///     let is_dark_mode = use_media_query("(prefers-color-scheme: dark)");
///
///     html! {
///         <>
///             <p>
///                 <b>{ "Is small screen (max-width: 600px): " }</b>
///                 { is_small_screen }
///             </p>
///             <p>
///                 <b>{ "Is dark mode preferred: " }</b>
///                 { is_dark_mode }
///             </p>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_media_query(query: &str) -> bool {
    let matches = use_state(|| false);

    {
        let matches = matches.clone();
        let query = query.to_string();

        use_effect_with(query, move |query| {
            let window = window();

            // Create the MediaQueryList object
            let mql = js_sys::Reflect::get(&window, &JsValue::from_str("matchMedia"))
                .ok()
                .and_then(|mm| mm.dyn_into::<js_sys::Function>().ok())
                .and_then(|f| f.call1(&window.into(), &JsValue::from_str(query)).ok());

            if let Some(mql_val) = mql {
                // Get initial match status
                let initial_matches = js_sys::Reflect::get(&mql_val, &JsValue::from_str("matches"))
                    .ok()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                matches.set(initial_matches);

                // Set up event listener for changes
                if let Ok(target) = mql_val.clone().dyn_into::<web_sys::EventTarget>() {
                    let matches_clone = matches.clone();
                    let mql_for_closure = mql_val.clone();

                    let closure = Closure::wrap(Box::new(move |_ev: JsValue| {
                        let current_matches =
                            js_sys::Reflect::get(&mql_for_closure, &JsValue::from_str("matches"))
                                .ok()
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);

                        matches_clone.set(current_matches);
                    }) as Box<dyn FnMut(JsValue)>);

                    let _ = target.add_event_listener_with_callback(
                        "change",
                        closure.as_ref().unchecked_ref(),
                    );

                    // Store the closure in a variable so it lives long enough
                    // The closure will be leaked (forgotten) to live for the page lifetime
                    // This is consistent with other hooks in the codebase
                    closure.forget();
                }
            }

            || ()
        });
    }

    // Also update on mount to ensure we have the latest value
    {
        let matches = matches.clone();
        let query = query.to_string();

        use_mount(move || {
            let window = window();

            // Re-check on mount
            let mql = js_sys::Reflect::get(&window, &JsValue::from_str("matchMedia"))
                .ok()
                .and_then(|mm| mm.dyn_into::<js_sys::Function>().ok())
                .and_then(|f| f.call1(&window.into(), &JsValue::from_str(&query)).ok());

            if let Some(mql_val) = mql {
                let current_matches = js_sys::Reflect::get(&mql_val, &JsValue::from_str("matches"))
                    .ok()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                matches.set(current_matches);
            }
        });
    }

    *matches
}
