use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::prelude::*;

use super::{use_mut_latest, use_unmount};

/// State handle for the [`use_timeout`] hook.
pub struct UseTimeoutHandle {
    reset: Rc<dyn Fn()>,
    cancel: Rc<dyn Fn()>,
}

impl UseTimeoutHandle {
    /// Reset the timeout.
    pub fn reset(&self) {
        (self.reset)();
    }

    /// Cancel the timeout.
    pub fn cancel(&self) {
        (self.cancel)();
    }
}

impl Clone for UseTimeoutHandle {
    fn clone(&self) -> Self {
        Self {
            reset: self.reset.clone(),
            cancel: self.cancel.clone(),
        }
    }
}

/// A hook that schedules a timeout to invoke `callback` in `millis` milliseconds from now.
/// The timeout will be cancelled if `millis` is set to 0 or `cancel()` is called.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Timeout)]
/// fn timeout() -> Html {
///     let state = use_state(|| 0);
///
///     let timeout = {
///         let state = state.clone();
///         use_timeout(move || {
///             state.set(*state + 1);
///         }, 2000)
///     };
///
///     let onreset = {
///         let timeout = timeout.clone();
///         Callback::from(move |_| timeout.reset())
///     };
///
///     let oncancel = {
///         let timeout = timeout.clone();
///         Callback::from(move |_| timeout.cancel())
///     };
///     
///     html! {
///         <>
///             <button onclick={onreset}>{ "Reset timeout" }</button>
///             <button onclick={oncancel}>{ "Cancel timeout" }</button>
///             { *state }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_timeout<Callback>(callback: Callback, millis: u32) -> UseTimeoutHandle
where
    Callback: FnOnce() + 'static,
{
    let callback_ref = use_mut_latest(Some(callback));
    let timeout_ref = use_mut_ref(|| None);

    let reset = {
        let timeout_ref = timeout_ref.clone();
        Rc::new(move || {
            let timeout_ref = timeout_ref.clone();
            let callback_ref = callback_ref.clone();
            if millis > 0 {
                *timeout_ref.borrow_mut() = Some(Timeout::new(millis, move || {
                    let callback_ref = callback_ref.current();
                    let callback = (*callback_ref.borrow_mut()).take();
                    if let Some(callback) = callback {
                        callback();
                    }
                }));
            } else {
                *timeout_ref.borrow_mut() = None;
            }
        })
    };

    let cancel = {
        let timeout_ref = timeout_ref.clone();
        Rc::new(move || {
            *timeout_ref.borrow_mut() = None;
        })
    };

    {
        let reset = reset.clone();
        let timeout_ref = timeout_ref.clone();
        use_effect_with_deps(
            move |_| {
                reset();

                move || *timeout_ref.borrow_mut() = None
            },
            millis,
        );
    }

    use_unmount(move || {
        *timeout_ref.borrow_mut() = None;
    });

    UseTimeoutHandle { reset, cancel }
}
