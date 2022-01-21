use gloo::timers::callback::Timeout;

use yew::prelude::*;

use super::use_mut_latest;

/// A hook that schedules a timeout to invoke `callback` in `millis` milliseconds from now.
/// The timeout will be cancelled if `millis` is set to 0.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_timeout;
///
/// #[function_component(Timeout)]
/// fn timeout() -> Html {
///     let state = use_state(|| 0);
///
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
/// }
/// ```
pub fn use_timeout<Callback>(callback: Callback, millis: u32)
where
    Callback: FnOnce() + 'static,
{
    let callback_ref = use_mut_latest(Some(callback));
    let timeout_ref = use_mut_ref(|| None);

    use_effect_with_deps(
        move |millis| {
            if *millis > 0 {
                *timeout_ref.borrow_mut() = Some(Timeout::new(*millis, move || {
                    let callback_ref = callback_ref.current();
                    let callback = (*callback_ref.borrow_mut()).take();
                    if let Some(callback) = callback {
                        callback();
                    }
                }));
            } else {
                *timeout_ref.borrow_mut() = None;
            }

            move || *timeout_ref.borrow_mut() = None
        },
        millis,
    );
}
