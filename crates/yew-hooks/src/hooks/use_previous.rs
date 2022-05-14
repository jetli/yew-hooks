use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

/// State handle for the [`use_previous`] hook.
pub struct UsePreviousHandle<T> {
    inner: Rc<T>,
}

impl<T> UsePreviousHandle<T> {
    /// Get the previous immutable ref to state or props.
    pub fn previous(&self) -> Rc<T> {
        self.inner.clone()
    }
}

impl<T> Deref for UsePreviousHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl<T> Clone for UsePreviousHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UsePreviousHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// This hook returns the previous immutable ref to state or props.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UsePrevious)]
/// fn previous() -> Html {
///     let state = use_state(|| 0);
///     let previous_state = use_previous(state.clone());
///
///
///     let onincrease = {
///         let state = state.clone();
///         Callback::from(move |_| state.set(*state + 1))
///     };
///     let ondecrease = {
///         let state = state.clone();
///         Callback::from(move |_| state.set(*state - 1))
///     };
///     
///     html! {
///         <div>
///             <button onclick={onincrease}>{ "Increase" }</button>
///             <button onclick={ondecrease}>{ "Decrease" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *state }
///             </p>
///             <p>
///                 <b>{ "Previous value: " }</b>
///                 { **previous_state }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_previous<T>(value: T) -> UsePreviousHandle<T>
where
    T: 'static,
{
    let value_rc = Rc::new(value);
    let state = use_mut_ref(|| value_rc.clone());

    // Update the ref each render so if it changes the newest value will be saved.
    let inner = state.replace(value_rc);

    UsePreviousHandle { inner }
}
