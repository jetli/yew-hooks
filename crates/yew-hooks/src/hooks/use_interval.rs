use gloo::timers::callback::Interval;
use yew::{use_effect_with_deps, use_mut_ref};

/// A hook that schedules an interval to invoke `callback` every `millis` milliseconds.
/// The interval will be cancelled if `millis` is set to 0.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// # use yew_hooks::use_interval;
/// #
/// #[function_component(Interval)]
/// fn interval() -> Html {
///     let state = use_state(|| 0);
///
///     {
///         let state = state.clone();
///         use_interval(move || {
///             state.set(*state + 1);
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
pub fn use_interval<Callback>(callback: Callback, millis: u32)
where
    Callback: FnMut() + 'static,
{
    let callback_ref = use_mut_ref(|| None);
    let callback_buffer_ref = use_mut_ref(|| None);
    let interval_ref = use_mut_ref(|| None);

    // Update the ref each render so if it changes the newest callback will be invoked.
    *callback_ref.borrow_mut() = Some(callback);

    use_effect_with_deps(
        move |millis| {
            if *millis > 0 {
                *interval_ref.borrow_mut() = Some(Interval::new(*millis, move || {
                    if let Some(callback) = (*callback_ref.borrow_mut()).take() {
                        *callback_buffer_ref.borrow_mut() = Some(callback);
                    }

                    if let Some(callback) = (*callback_buffer_ref.borrow_mut()).as_mut() {
                        callback();
                    }
                }));
            } else {
                *interval_ref.borrow_mut() = None;
            }

            move || *interval_ref.borrow_mut() = None
        },
        millis,
    );
}
