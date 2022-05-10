use super::{use_timeout, UseTimeoutHandle};

/// State handle for the [`use_debounce`] hook.
pub struct UseDebounceHandle {
    inner: UseTimeoutHandle,
}

impl UseDebounceHandle {
    /// Run the debounce.
    pub fn run(&self) {
        self.inner.reset()
    }

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

/// A hook that delays invoking a function until after wait milliseconds have elapsed
/// since the last time the debounced function was invoked.
///
/// # Example
///
/// ```rust
/// # use web_sys::HtmlInputElement;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Debounce)]
/// fn debounce() -> Html {
///     let status = use_state(|| "Typing stopped".to_string());
///     let value = use_state(|| "".to_string());
///     let debounced_value = use_state(|| "".to_string());
///     
///     let debounce = {
///         let value = value.clone();
///         let status = status.clone();
///         let debounced_value = debounced_value.clone();
///         use_debounce(
///             move || {
///                 debounced_value.set((*value).clone());
///                 status.set("Typing stopped".to_string());
///             },
///             2000,
///         )
///     };
///     
///     let oninput = {
///         let status = status.clone();
///         let value = value.clone();
///         let debounce = debounce.clone();
///         Callback::from(move |e: InputEvent| {
///             let input: HtmlInputElement = e.target_unchecked_into();
///             value.set(input.value());
///             status.set("Waiting for typing to stop...".to_string());
///             debounce.run();
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
pub fn use_debounce<Callback>(callback: Callback, millis: u32) -> UseDebounceHandle
where
    Callback: FnOnce() + 'static,
{
    let inner = use_timeout(callback, millis);

    UseDebounceHandle { inner }
}
