use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

struct UseStatePtrEqReducer<T> {
    value: Rc<T>,
}

impl<T> Reducible for UseStatePtrEqReducer<T> {
    type Action = T;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(Self {
            value: action.into(),
        })
    }
}

impl<T> PartialEq for UseStatePtrEqReducer<T> {
    fn eq(&self, rhs: &Self) -> bool {
        // Check if the two `Rc`s point to the same allocation, instead of PartialEq of the values.
        Rc::ptr_eq(&self.value, &rhs.value)
    }
}

/// State handle for the [`use_state_ptr_eq`] hook.
pub struct UseStatePtrEqHandle<T> {
    inner: UseReducerHandle<UseStatePtrEqReducer<T>>,
}

impl<T> UseStatePtrEqHandle<T> {
    /// Replaces the value
    pub fn set(&self, value: T) {
        self.inner.dispatch(value)
    }
}

impl<T> Deref for UseStatePtrEqHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner).value
    }
}

impl<T> Clone for UseStatePtrEqHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseStatePtrEqHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

/// Similar to `use_state_eq`, but check if the two `Rc`s of values point to the same allocation,
/// instead of PartialEq of the values.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseStatePtrEq)]
/// fn state_ptr_eq() -> Html {
///     let state = use_state_ptr_eq(|| "".to_string());
///
///     let onclick = {
///         let state = state.clone();
///         Callback::from(move |_| {
///             state.set("Hello, world!".to_string());
///         })
///     };
///     
///     html! {
///         <>
///             <button {onclick}>{ "Hello, world!" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { &*state }
///             </p>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_state_ptr_eq<T, F>(init_fn: F) -> UseStatePtrEqHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let handle = use_reducer_eq(move || UseStatePtrEqReducer {
        value: Rc::new(init_fn()),
    });

    UseStatePtrEqHandle { inner: handle }
}
