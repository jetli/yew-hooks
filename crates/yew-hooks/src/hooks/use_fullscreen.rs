use std::rc::Rc;

use gloo::events::EventListener;
use gloo::utils::document;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

use super::{use_state_ptr_eq, UseStatePtrEqHandle};

/// State handle for the [`use_fullscreen`] hook.
pub struct UseFullscreenHandle {
    /// Whether fullscreen is currently active.
    pub is_fullscreen: UseStatePtrEqHandle<bool>,
    /// Whether the Fullscreen API is supported in the current browser.
    pub is_supported: Rc<bool>,

    enter: Rc<dyn Fn()>,
    exit: Rc<dyn Fn()>,
    toggle: Rc<dyn Fn()>,
}

impl UseFullscreenHandle {
    /// Enter fullscreen mode for the element.
    pub fn enter(&self) {
        (self.enter)();
    }

    /// Exit fullscreen mode.
    pub fn exit(&self) {
        (self.exit)();
    }

    /// Toggle fullscreen mode for the element.
    /// If currently in fullscreen, exits fullscreen.
    /// If not in fullscreen, enters fullscreen for the element.
    pub fn toggle(&self) {
        (self.toggle)();
    }
}

impl Clone for UseFullscreenHandle {
    fn clone(&self) -> Self {
        Self {
            is_fullscreen: self.is_fullscreen.clone(),
            is_supported: self.is_supported.clone(),
            enter: self.enter.clone(),
            exit: self.exit.clone(),
            toggle: self.toggle.clone(),
        }
    }
}

/// This hook provides functionality to control fullscreen mode for elements.
/// It allows entering, exiting, and toggling fullscreen mode, and tracks
/// the current fullscreen state and element.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseFullscreen)]
/// fn fullscreen() -> Html {
///     let element_ref = use_node_ref();
///     let fullscreen = use_fullscreen(element_ref.clone());
///
///     let onenter = {
///         let fullscreen = fullscreen.clone();
///         Callback::from(move |_| {
///             fullscreen.enter();
///         })
///     };
///
///     let onexit = {
///         let fullscreen = fullscreen.clone();
///         Callback::from(move |_| {
///             fullscreen.exit();
///         })
///     };
///
///     let ontoggle = {
///         let fullscreen = fullscreen.clone();
///         Callback::from(move |_| {
///             fullscreen.toggle();
///         })
///     };
///
///     html! {
///         <div>
///             <div ref={element_ref} style="width: 100%; height: 300px; background-color: lightblue; display: flex; align-items: center; justify-content: center;">
///                 <p>{ "This element can go fullscreen" }</p>
///             </div>
///             <div>
///                 <button onclick={onenter} disabled={*fullscreen.is_fullscreen}>
///                     { "Enter Fullscreen" }
///                 </button>
///                 <button onclick={onexit} disabled={!*fullscreen.is_fullscreen}>
///                     { "Exit Fullscreen" }
///                 </button>
///                 <button onclick={ontoggle}>
///                     { "Toggle Fullscreen" }
///                 </button>
///             </div>
///             <p>{ format!("Is fullscreen: {}", *fullscreen.is_fullscreen) }</p>
///             <p>{ format!("Is supported: {}", *fullscreen.is_supported) }</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_fullscreen(node: NodeRef) -> UseFullscreenHandle {
    let is_fullscreen = use_state_ptr_eq(|| false);

    // Check if Fullscreen API is supported
    let is_supported = use_memo((), |_| {
        let doc = document();
        // Check if fullscreen API is available
        js_sys::Reflect::has(&doc, &JsValue::from("fullscreenEnabled")).unwrap_or(false)
    });

    // Function to get the current fullscreen element
    let get_fullscreen_element = || {
        let doc = document();
        // Try to get fullscreen element using reflection
        js_sys::Reflect::get(&doc, &JsValue::from("fullscreenElement"))
            .ok()
            .and_then(|v| v.dyn_into::<web_sys::Element>().ok())
    };

    // Update state based on current fullscreen status
    let update_fullscreen_state = {
        let is_fullscreen = is_fullscreen.clone();
        move || {
            let fullscreen_element = get_fullscreen_element();
            let is_fs = fullscreen_element.is_some();
            is_fullscreen.set(is_fs);
        }
    };

    // Set up event listeners for fullscreen change events
    {
        let update_fullscreen_state1 = update_fullscreen_state.clone();
        let update_fullscreen_state2 = update_fullscreen_state.clone();
        use_effect_with((), move |_| {
            let doc = document();
            let listener = EventListener::new(&doc, "fullscreenchange", move |_| {
                update_fullscreen_state1();
            });

            // Also listen for webkit-specific events for Safari compatibility
            let webkit_listener = EventListener::new(&doc, "webkitfullscreenchange", move |_| {
                update_fullscreen_state2();
            });

            // Keep listeners alive for the duration of the component
            Box::new(move || {
                drop(listener);
                drop(webkit_listener);
            })
        });
    }

    // Initial state update
    {
        let update_fullscreen_state = update_fullscreen_state.clone();
        use_effect_with((), move |_| {
            update_fullscreen_state();
            || ()
        });
    }

    let enter = {
        let is_supported = is_supported.clone();
        let node = node.clone();
        Rc::new(move || {
            if !*is_supported {
                return;
            }

            let Some(element) = node.cast::<HtmlElement>() else {
                // Only HtmlElement can go fullscreen
                return;
            };

            // Try standard method first using reflection
            let result = js_sys::Reflect::get(&element, &JsValue::from("requestFullscreen"));
            if let Ok(func) = result {
                if func.is_function() {
                    let func = js_sys::Function::from(func);
                    let _ = func.call0(&element);
                } else {
                    // Fallback to webkit method for Safari
                    let func =
                        js_sys::Reflect::get(&element, &JsValue::from("webkitRequestFullscreen"));
                    if let Ok(func) = func {
                        if func.is_function() {
                            let func = js_sys::Function::from(func);
                            let _ = func.call0(&element);
                        }
                    }
                }
            } else {
                // Fallback to webkit method for Safari
                let func =
                    js_sys::Reflect::get(&element, &JsValue::from("webkitRequestFullscreen"));
                if let Ok(func) = func {
                    if func.is_function() {
                        let func = js_sys::Function::from(func);
                        let _ = func.call0(&element);
                    }
                }
            }
        })
    };

    let exit = {
        let is_supported = is_supported.clone();
        Rc::new(move || {
            if !*is_supported {
                return;
            }

            let doc = document();
            // Try standard method first using reflection
            let result = js_sys::Reflect::get(&doc, &JsValue::from("exitFullscreen"));
            if let Ok(func) = result {
                if func.is_function() {
                    let func = js_sys::Function::from(func);
                    let _ = func.call0(&doc);
                } else {
                    // Fallback to webkit method for Safari
                    let func = js_sys::Reflect::get(&doc, &JsValue::from("webkitExitFullscreen"));
                    if let Ok(func) = func {
                        if func.is_function() {
                            let func = js_sys::Function::from(func);
                            let _ = func.call0(&doc);
                        }
                    }
                }
            } else {
                // Fallback to webkit method for Safari
                let func = js_sys::Reflect::get(&doc, &JsValue::from("webkitExitFullscreen"));
                if let Ok(func) = func {
                    if func.is_function() {
                        let func = js_sys::Function::from(func);
                        let _ = func.call0(&doc);
                    }
                }
            }
        })
    };

    let toggle = {
        let is_fullscreen = is_fullscreen.clone();
        let enter = enter.clone();
        let exit = exit.clone();
        Rc::new(move || {
            if *is_fullscreen {
                exit();
            } else {
                enter();
            }
        })
    };

    UseFullscreenHandle {
        is_fullscreen,
        is_supported,
        enter,
        exit,
        toggle,
    }
}
