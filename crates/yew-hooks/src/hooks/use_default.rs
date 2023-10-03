use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

/// State handle for the [`use_default`] hook.
pub struct UseDefaultHandle<T> {
    inner: UseStateHandle<Option<T>>,
    default: Rc<T>,
}

impl<T> UseDefaultHandle<T>
where
    T: 'static,
{
    /// Replaces the value.
    pub fn set(&self, value: Option<T>) {
        self.inner.set(value);
    }
}

impl<T> Deref for UseDefaultHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let value = &(*self.inner);
        value.as_ref().unwrap_or_else(|| self.default.as_ref())
    }
}

impl<T> Clone for UseDefaultHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            default: self.default.clone(),
        }
    }
}

impl<T> PartialEq for UseDefaultHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A state hook that returns the default value when state is None.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseDefault)]
/// fn default() -> Html {
///     let state = use_default(|| None, "Hello(default)".to_string());
///
///     let onclick = {
///         let state = state.clone();
///         Callback::from(move |_| {
///             state.set(Some("World!".to_string()));
///         })
///     };
///
///     let onclear = {
///         let state = state.clone();
///         Callback::from(move |_| {
///             state.set(None);
///         })
///     };
///     
///     html! {
///         <>
///             <button {onclick}>{ "Set to World!" }</button>
///             <button onclick={onclear}>{ "Clear" }</button>
///             <b>{ "Current value: " }</b>
///             { &*state }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_default<T, F>(init_fn: F, default: T) -> UseDefaultHandle<T>
where
    T: 'static,
    F: FnOnce() -> Option<T>,
{
    let inner = use_state(init_fn);
    let default = use_memo((), |_| default);

    UseDefaultHandle { inner, default }
}
