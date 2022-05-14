use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

/// State handle for the [`use_mut_latest`] hook.
pub struct UseMutLatestHandle<T> {
    inner: Rc<RefCell<Rc<RefCell<T>>>>,
}

impl<T> UseMutLatestHandle<T> {
    /// Get the latest mutable ref to state or props.
    pub fn current(&self) -> Rc<RefCell<T>> {
        self.inner.borrow().clone()
    }
}

impl<T> Clone for UseMutLatestHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseMutLatestHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// State handle for the [`use_latest`] hook.
pub struct UseLatestHandle<T> {
    inner: Rc<RefCell<Rc<T>>>,
}

impl<T> UseLatestHandle<T> {
    /// Get the latest immutable ref to state or props.
    pub fn current(&self) -> Rc<T> {
        self.inner.borrow().clone()
    }
}

impl<T> Clone for UseLatestHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseLatestHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// This hook returns the latest mutable ref to state or props.
///
/// # Example
///
/// ```rust
/// # use gloo::timers::callback::Interval;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseMutLatest)]
/// fn mut_latest() -> Html {
///     let state = use_state(|| 0);
///     let interval = use_mut_ref(|| None);
///     let closure = {
///         let state = state.clone();
///         move || state.set(*state + 1)
///     };
///
///     let latest_closure = use_mut_latest(closure);
///
///     use_effect_with_deps(move |_| {
///         *interval.borrow_mut() = Some(Interval::new(1000, move || {
///             let latest_closure = latest_closure.current();
///             let closure = &*latest_closure.borrow_mut();
///             // This will get the latest closure and increase state by 1 each time.
///             closure();
///         }));
///         move || *interval.borrow_mut() = None
///     }, ());
///     
///     html! {
///         <div>
///             <p>
///                 <b>{ "Latest value: " }</b>
///                 { *state }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_mut_latest<T>(value: T) -> UseMutLatestHandle<T>
where
    T: 'static,
{
    let value_rc = Rc::new(RefCell::new(value));
    let inner = use_mut_ref(|| value_rc.clone());

    // Update the ref each render so if it changes the newest value will be saved.
    *inner.borrow_mut() = value_rc;

    UseMutLatestHandle { inner }
}

/// This hook returns the latest immutable ref to state or props.
///
/// # Example
///
/// ```rust
/// # use gloo::timers::callback::Interval;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseLatest)]
/// fn latest() -> Html {
///     let state = use_state(|| 0);
///     let interval = use_mut_ref(|| None);
///
///     let latest_state = use_latest(state.clone());
///
///     {
///         let state = state.clone();
///         use_effect_with_deps(move |_| {
///             *interval.borrow_mut() = Some(Interval::new(1000, move || {
///                 // This will get the latest state and increase it by 1 each time.
///                 state.set(**latest_state.current() + 1);
///             }));
///             move || *interval.borrow_mut() = None
///         }, ());
///     }
///     
///     html! {
///         <div>
///             <p>
///                 <b>{ "Latest value: " }</b>
///                 { *state }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_latest<T>(value: T) -> UseLatestHandle<T>
where
    T: 'static,
{
    let value_rc = Rc::new(value);
    let inner = use_mut_ref(|| value_rc.clone());

    // Update the ref each render so if it changes the newest value will be saved.
    *inner.borrow_mut() = value_rc;

    UseLatestHandle { inner }
}
