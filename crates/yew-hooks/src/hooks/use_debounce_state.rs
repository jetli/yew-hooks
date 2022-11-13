use std::ops::Deref;
use std::rc::Rc;

use yew::prelude::*;

use super::use_debounce;

/// State handle for the [`use_debounce_state`] hook.
pub struct UseDebounceStateHandle<T> {
    inner: UseStateHandle<T>,
    set: Rc<dyn Fn(T)>,
}

impl<T> UseDebounceStateHandle<T> {
    // Set the value.
    pub fn set(&self, value: T) {
        (self.set)(value)
    }
}

impl<T> Deref for UseDebounceStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseDebounceStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            set: self.set.clone(),
        }
    }
}

impl<T> PartialEq for UseDebounceStateHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A hook that delays updating state until after wait milliseconds have elapsed
/// since the last time state was updated.
///
/// # Example
///
/// ```rust
/// # use web_sys::HtmlInputElement;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(DebounceState)]
/// fn debounce_state() -> Html {
///     let state = use_debounce_state(|| "".to_string(), 2000);
///
///     let oninput = {
///         let state = state.clone();
///         Callback::from(move |e: InputEvent| {
///             let input: HtmlInputElement = e.target_unchecked_into();
///             state.set(input.value());
///         })
///     };
///
///     html! {
///         <>
///             <input type="text" placeholder="Debounced input" {oninput}/>
///             <b>{ "Debounced state: " }</b> {&*state}
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_debounce_state<T, F>(init_fn: F, millis: u32) -> UseDebounceStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let value = use_mut_ref(|| None);
    let inner = use_state(init_fn);
    let debounce = {
        let value = value.clone();
        let inner = inner.clone();
        use_debounce(
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
            debounce.run();
        })
    };

    UseDebounceStateHandle { inner, set }
}
