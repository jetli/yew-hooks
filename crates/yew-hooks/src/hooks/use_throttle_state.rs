use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

use super::use_throttle;

/// State handle for the [`use_throttle_state`] hook.
pub struct UseThrottleStateHandle<T> {
    inner: UseStateHandle<T>,
    set: Rc<dyn Fn(T)>,
}

impl<T> UseThrottleStateHandle<T> {
    // Set the value.
    pub fn set(&self, value: T) {
        (self.set)(value);
    }
}

impl<T> Deref for UseThrottleStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseThrottleStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            set: self.set.clone(),
        }
    }
}

impl<T> PartialEq for UseThrottleStateHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A hook that throttles updating state, the state is only updated once every `millis`.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(ThrottleState)]
/// fn throttle_state() -> Html {
///     let state = use_throttle_state(|| 0, 2000);
///
///     let onclick = {
///         let state = state.clone();
///         Callback::from(move |_| state.set(*state + 1))
///     };
///
///     html! {
///         <>
///             <button {onclick}>{ "Click fast!" }</button>
///             <b>{ "State: " }</b> {*state}
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_throttle_state<T, F>(init_fn: F, millis: u32) -> UseThrottleStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let value = use_mut_ref(|| None);
    let inner = use_state(init_fn);
    let throttle = {
        let value = value.clone();
        let inner = inner.clone();
        use_throttle(
            move || {
                let value = (*value.borrow_mut()).take();
                if let Some(value) = value {
                    inner.set(value);
                }
            },
            millis,
        )
    };

    let set = {
        Rc::new(move |new_value: T| {
            *value.borrow_mut() = Some(new_value);
            throttle.run();
        })
    };

    UseThrottleStateHandle { inner, set }
}
