use gloo::timers::callback::Timeout;
use yew::{use_effect_with_deps, use_mut_ref};

/// A hook that schedules a timeout to invoke `callback` in `millis` milliseconds from now.
/// The timeout will be cancelled if `millis` is set to 0.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// #
/// # use yew_hooks::use_timeout;
/// #
/// #[function_component(Timeout)]
/// fn timeout() -> Html {
///     let state = use_state(|| 0);
///     {
///         let state = state.clone();
///         use_timeout(move || {
///             state.set(1);
///         }, 2000);
///     }
///     
///     html! {
///         <>
///             { *state }
///         </>
///     }
///
/// }
/// ```
pub fn use_timeout<Callback>(callback: Callback, millis: u32)
where
    Callback: FnOnce() + 'static,
{
    let callback_ref = use_mut_ref(|| None);
    let timeout_ref = use_mut_ref(|| None);

    // Update the ref each render so if it changes the newest callback will be invoked.
    *callback_ref.borrow_mut() = Some(callback);

    use_effect_with_deps(
        move |millis| {
            if *millis > 0 {
                if let Some(callback) = (*callback_ref.borrow_mut()).take() {
                    *timeout_ref.borrow_mut() = Some(Timeout::new(*millis, callback));
                }
            } else {
                *timeout_ref.borrow_mut() = None;
            }

            move || *timeout_ref.borrow_mut() = None
        },
        millis,
    );
}
