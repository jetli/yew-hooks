use super::{use_effect_once, use_mut_latest};

/// A lifecycle hook that calls a function when the component will unmount.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
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
/// }
/// ```
pub fn use_unmount<Callback>(callback: Callback)
where
    Callback: FnOnce() + 'static,
{
    let callback_ref = use_mut_latest(Some(callback));

    use_effect_once(move || {
        move || {
            let callback_ref = callback_ref.current();
            let callback = (*callback_ref.borrow_mut()).take();
            if let Some(callback) = callback {
                callback();
            }
        }
    });
}
