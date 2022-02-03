use web_sys::BeforeUnloadEvent;

use super::use_event_with_window;

/// A side-effect hook that shows browser alert when user try to reload or close the page.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_before_unload;
///
/// #[function_component(UseBeforeUnload)]
/// fn before_unload() -> Html {
///     use_before_unload(true, "You have unsaved changes, are you sure?".to_string());
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
pub fn use_before_unload(enabled: bool, msg: String) {
    use_event_with_window("beforeunload", move |e: BeforeUnloadEvent| {
        if !enabled {
            return;
        }

        if !msg.is_empty() {
            e.set_return_value(msg.as_str());
        }

        // WebKit-derived browsers don't follow the spec for the dialog box.
        // We should return msg in the future for the event handler.
        // https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent
    })
}
