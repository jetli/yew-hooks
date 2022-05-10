use std::rc::Rc;

use yew::prelude::*;

use super::{use_mut_latest, use_timeout};

/// State handle for the [`use_throttle`] hook.
pub struct UseThrottleHandle {
    run: Rc<dyn Fn()>,
    cancel: Rc<dyn Fn()>,
}

impl UseThrottleHandle {
    /// Run the throttle.
    pub fn run(&self) {
        (self.run)()
    }

    /// Cancel the throttle.
    pub fn cancel(&self) {
        (self.cancel)()
    }
}

impl Clone for UseThrottleHandle {
    fn clone(&self) -> Self {
        Self {
            run: self.run.clone(),
            cancel: self.cancel.clone(),
        }
    }
}

/// A hook that throttles invoking a function, the function is only executed once every `millis`.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Throttle)]
/// fn throttle() -> Html {
///     let state = use_state(|| 0);
///
///     let throttle = {
///         let state = state.clone();
///         use_throttle(
///             move || {
///                 state.set(*state + 1);
///             },
///             2000,
///         )
///     };
///
///     let onclick = {
///         let throttle = throttle.clone();
///         Callback::from(move |_| throttle.run())
///     };
///
///     let oncancel = { Callback::from(move |_| throttle.cancel()) };
///
///     html! {
///         <>
///             <button {onclick}>{ "Click fast!" }</button>
///             <button onclick={oncancel}>{ "Cancel throttle" }</button>
///             <b>{ "State: " }</b> {*state}
///         </>
///     }
/// }
/// ```
pub fn use_throttle<Callback>(callback: Callback, millis: u32) -> UseThrottleHandle
where
    Callback: FnMut() + 'static,
{
    let throttled = use_mut_ref(|| false);
    let callback_ref = use_mut_latest(callback);
    let timeout = {
        let throttled = throttled.clone();
        use_timeout(
            move || {
                *throttled.borrow_mut() = false;
            },
            millis,
        )
    };

    let run = {
        let throttled = throttled.clone();
        let timeout = timeout.clone();
        Rc::new(move || {
            let throttled_value = *throttled.borrow();
            if !throttled_value {
                let callback_ref = callback_ref.current();
                let callback = &mut *callback_ref.borrow_mut();
                callback();
                *throttled.borrow_mut() = true;
                timeout.reset();
            }
        })
    };

    let cancel = {
        Rc::new(move || {
            timeout.cancel();
            *throttled.borrow_mut() = false;
        })
    };

    UseThrottleHandle { run, cancel }
}
