use yew::use_mut_ref;

use super::use_effect_once;

/// A lifecycle hook that calls a function when the component will unmount.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// # use yew_hooks::use_unmount;
/// #
/// #[function_component(Unmount)]
/// fn unmount() -> Html {
///     use_unmount(|| {
///         debug!("Running clean-up of effect on unmount");
///     });
///     
///     html! {
///         <>
///         </>
///     }
///
/// }
/// ```
pub fn use_unmount<Callback>(callback: Callback)
where
    Callback: FnOnce() + 'static,
{
    let callback_ref = use_mut_ref(|| None);

    // Update the ref each render so if it changes the newest callback will be invoked.
    *callback_ref.borrow_mut() = Some(callback);

    use_effect_once(move || {
        move || {
            if let Some(callback) = (*callback_ref.borrow_mut()).take() {
                callback();
            }
        }
    });
}
