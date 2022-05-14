use gloo::timers::callback::Interval;
use yew::prelude::*;

use super::use_mut_latest;

/// A hook that schedules an interval to invoke `callback` every `millis` milliseconds.
/// The interval will be cancelled if `millis` is set to 0.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
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
#[hook]
pub fn use_interval<Callback>(callback: Callback, millis: u32)
where
    Callback: FnMut() + 'static,
{
    let callback_ref = use_mut_latest(callback);
    let interval_ref = use_mut_ref(|| None);

    use_effect_with_deps(
        move |millis| {
            if *millis > 0 {
                *interval_ref.borrow_mut() = Some(Interval::new(*millis, move || {
                    let callback_ref = callback_ref.current();
                    let callback = &mut *callback_ref.borrow_mut();
                    callback();
                }));
            } else {
                *interval_ref.borrow_mut() = None;
            }

            move || *interval_ref.borrow_mut() = None
        },
        millis,
    );
}
