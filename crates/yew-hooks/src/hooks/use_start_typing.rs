use std::borrow::Cow;

use gloo::events::EventListener;
use gloo::utils::window;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement, KeyboardEvent};
use yew::prelude::*;

use super::use_latest;

/// Type alias for key filter function to reduce type complexity
type KeyFilter = Box<dyn Fn(&str) -> bool>;

/// A hook that triggers a callback when the user starts typing on the page
/// without an editable element focused.
///
/// The callback only fires when:
/// - No editable element (`<input>`, `<textarea>`, or `contenteditable`) is focused
/// - The pressed key is alphanumeric (A-Z, 0-9)
/// - No modifier keys (Ctrl, Alt, Meta) are held
///
/// This allows users to start typing anywhere on the page without accidentally
/// triggering the callback when using keyboard shortcuts or interacting with form fields.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseStartTyping)]
/// fn start_typing() -> Html {
///     use_start_typing(move |event: KeyboardEvent| {
///         debug!("Started typing with key: {}", event.key());
///     });
///
///     html! {
///         <div>
///             <p>{ "Try typing anywhere on the page (without focusing on an input field)." }</p>
///             <input type="text" placeholder="Focus here and typing won't trigger the callback" />
///             <textarea placeholder="Same for textarea" />
///             <div contenteditable="true" style="border: 1px solid #ccc; padding: 8px; margin-top: 8px;">
///                 { "This is a contenteditable div. Typing here won't trigger the callback either." }
///             </div>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_start_typing<F>(callback: F)
where
    F: Fn(KeyboardEvent) + 'static,
{
    use_start_typing_with_options(callback, UseStartTypingOptions::default())
}

pub struct UseStartTypingOptions {
    /// The keyboard event type to listen for. Default: "keydown"
    pub event_type: Cow<'static, str>,
    /// Whether to check for editable elements. Default: true
    pub check_editable: bool,
    /// Whether to check for modifier keys. Default: true
    pub check_modifiers: bool,
    /// Custom function to determine if a key should trigger the callback.
    /// If not provided, defaults to checking if the key is alphanumeric.
    pub key_filter: Option<KeyFilter>,
}

impl Default for UseStartTypingOptions {
    fn default() -> Self {
        Self {
            event_type: "keydown".into(),
            check_editable: true,
            check_modifiers: true,
            key_filter: None,
        }
    }
}

impl Clone for UseStartTypingOptions {
    fn clone(&self) -> Self {
        Self {
            event_type: self.event_type.clone(),
            check_editable: self.check_editable,
            check_modifiers: self.check_modifiers,
            key_filter: None, // Can't clone function pointers, so we set to None
        }
    }
}

impl PartialEq for UseStartTypingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.event_type == other.event_type
            && self.check_editable == other.check_editable
            && self.check_modifiers == other.check_modifiers
            // We can't compare function pointers, so we just compare if both have Some or None
            && self.key_filter.is_some() == other.key_filter.is_some()
    }
}

impl std::fmt::Debug for UseStartTypingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseStartTypingOptions")
            .field("event_type", &self.event_type)
            .field("check_editable", &self.check_editable)
            .field("check_modifiers", &self.check_modifiers)
            .field(
                "key_filter",
                &if self.key_filter.is_some() {
                    "Some(...)"
                } else {
                    "None"
                },
            )
            .finish()
    }
}

/// A hook that triggers a callback when the user starts typing on the page
/// without an editable element focused, with custom event type.
///
/// This is similar to [`use_start_typing`] but allows specifying a custom event type.
/// The callback only fires when:
/// - No editable element (`<input>`, `<textarea>`, or `contenteditable`) is focused
/// - The pressed key matches the provided event type pattern
/// - No modifier keys (Ctrl, Alt, Meta) are held
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseStartTypingWithOptions)]
/// fn start_typing_with_options() -> Html {
///     use_start_typing_with_options(
///         move |event: KeyboardEvent| {
///             debug!("Started typing with key: {}", event.key());
///         },
///         UseStartTypingOptions {
///             event_type: "keypress".into(),
///             ..Default::default()
///         },
///     );
///
///     html! {
///         <div>
///             <p>{ "Try typing anywhere on the page (without focusing on an input field)." }</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_start_typing_with_options<F>(callback: F, options: UseStartTypingOptions)
where
    F: Fn(KeyboardEvent) + 'static,
{
    let callback = use_latest(callback);
    let options_ref = use_latest(options);

    use_effect_with((), move |_| {
        let window = window();

        // Helper function to check if an element is editable
        fn is_editable_element(element: &web_sys::Element) -> bool {
            if let Ok(input) = element.clone().dyn_into::<HtmlInputElement>() {
                // Check if input is not disabled and is visible (not hidden)
                !input.disabled() && input.type_() != "hidden"
            } else if let Ok(textarea) = element.clone().dyn_into::<HtmlTextAreaElement>() {
                // Check if textarea is not disabled
                !textarea.disabled()
            } else if let Ok(html_element) = element.clone().dyn_into::<HtmlElement>() {
                // Check if element has contenteditable attribute set to true
                html_element
                    .get_attribute("contenteditable")
                    .map(|value| value == "true")
                    .unwrap_or(false)
            } else {
                false
            }
        }

        // Check if the currently focused element is editable
        let is_editable_element_focused = {
            let window = window.clone();
            let options_ref = options_ref.clone();
            move || {
                if !options_ref.current().check_editable {
                    return false;
                }

                let document = match window.document() {
                    Some(doc) => doc,
                    None => return false,
                };
                let active_element = document.active_element();

                if let Some(element) = active_element {
                    is_editable_element(&element)
                } else {
                    false
                }
            }
        };

        let event_type = options_ref.current().event_type.clone();
        let options_ref_clone = options_ref.clone();
        let listener = EventListener::new(&window, event_type, move |event| {
            let keyboard_event: KeyboardEvent = JsValue::from(event).into();
            let options = &*options_ref_clone.current();

            // Check if the event should trigger
            if should_trigger_keyboard_event(&keyboard_event, options, &is_editable_element_focused)
            {
                (*callback.current())(keyboard_event);
            }
        });

        move || drop(listener)
    });
}

/// Helper function to determine if a keyboard event should trigger the callback
fn should_trigger_keyboard_event(
    keyboard_event: &KeyboardEvent,
    options: &UseStartTypingOptions,
    is_editable_element_focused: &dyn Fn() -> bool,
) -> bool {
    // Check modifier keys if enabled
    if options.check_modifiers
        && (keyboard_event.ctrl_key()
            || keyboard_event.alt_key()
            || keyboard_event.meta_key()
            || keyboard_event.shift_key())
    {
        return false;
    }

    // Check if editable element is focused if enabled
    if options.check_editable && is_editable_element_focused() {
        return false;
    }

    // Check if key passes the filter
    let key = keyboard_event.key();
    if let Some(filter) = &options.key_filter {
        filter(&key)
    } else {
        // Default key filter checks for alphanumeric keys
        if key.len() == 1 {
            let c = match key.chars().next() {
                Some(c) => c,
                None => return false,
            };
            c.is_ascii_alphanumeric()
        } else {
            false
        }
    }
}
