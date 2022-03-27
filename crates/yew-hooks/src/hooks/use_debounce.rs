use yew::prelude::*;

use super::{use_timeout, UseTimeoutHandle};

/// State handle for the [`use_debounce`] hook.
pub struct UseDebounceHandle {
    inner: UseTimeoutHandle,
}

impl UseDebounceHandle {
    /// Cancel the debounce.
    pub fn cancel(&self) {
        self.inner.cancel()
    }
}

impl Clone for UseDebounceHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// A hook that shat delays invoking a function until after wait milliseconds have elapsed
/// since the last time the debounced function was invoked.
/// The debounce timeout will start when the dependents changes.
///
/// # Example
///
/// ```rust
/// # use web_sys::HtmlInputElement;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_debounce;
///
/// #[function_component(Debounce)]
/// fn debounce() -> Html {
///     let status = use_state(|| "Typing stopped".to_string());
///     let value = use_state(|| "".to_string());
///     let debounced_value = use_state(|| "".to_string());
///     
///     let debounce = {
///         let value = value.clone();
///         let value2 = value.clone();
///         let status = status.clone();
///         let debounced_value = debounced_value.clone();
///         use_debounce(
///             move || {
///                 debounced_value.set((*value2).clone());
///                 status.set("Typing stopped".to_string());
///             },
///             2000,
///             value,
///         )
///     };
///     
///     let oninput = {
///         let status = status.clone();
///         let value = value.clone();
///         Callback::from(move |e: InputEvent| {
///             let input: HtmlInputElement = e.target_unchecked_into();
///             value.set(input.value());
///             status.set("Waiting for typing to stop...".to_string());
///         })
///     };
///     
///     let onclick = { Callback::from(move |_| debounce.cancel()) };
///     
///     html! {
///         <>
///             <input type="text" value={(*value).clone()} placeholder="Debounced input" {oninput}/>
///             <button {onclick}>{ "Cancel debounce" }</button>
///             <p>{&*status}</p>
///             <p>
///                 <b>{ "Value: " }</b> {&*value}
///             </p>
///             <p>
///                 <b>{ "Debounced value: " }</b> {&*debounced_value}
///             </p>
///         </>
///     }
/// }
/// ```
pub fn use_debounce<Callback, Dependents>(
    callback: Callback,
    millis: u32,
    deps: Dependents,
) -> UseDebounceHandle
where
    Callback: FnOnce() + 'static,
    Dependents: PartialEq + 'static,
{
    let inner = use_timeout(callback, millis);

    {
        let inner = inner.clone();
        use_effect_with_deps(
            move |_| {
                inner.reset();

                || ()
            },
            deps,
        );
    }

    UseDebounceHandle { inner }
}
