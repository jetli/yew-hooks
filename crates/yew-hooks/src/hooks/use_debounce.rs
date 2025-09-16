use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::prelude::*;

use super::use_unmount;

/// State handle for the [`use_debounce`] hook.
#[derive(Clone)]
pub struct UseDebounceHandle {
    run: Rc<dyn Fn()>,
    cancel: Rc<dyn Fn()>,
}

impl UseDebounceHandle {
    /// Run the debounce.
    pub fn run(&self) {
        (self.run)();
    }

    /// Cancel the debounce.
    pub fn cancel(&self) {
        (self.cancel)();
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
#[hook]
pub fn use_debounce<Callback>(callback: Callback, millis: u32) -> UseDebounceHandle
where
    Callback: Fn() + 'static,
{
    let callback_ref = Rc::new(callback);
    let timeout_ref = use_mut_ref(|| None::<Timeout>);

    let run = {
        let timeout_ref = timeout_ref.clone();
        let callback_ref = callback_ref.clone();
        Rc::new(move || {
            // Cancel any existing timeout
            if let Some(timeout) = timeout_ref.borrow_mut().take() {
                timeout.cancel();
            }

            // Create new timeout
            let callback_ref = callback_ref.clone();
            *timeout_ref.borrow_mut() = Some(Timeout::new(millis, move || {
                callback_ref();
            }));
        })
    };

    let cancel = {
        let timeout_ref = timeout_ref.clone();
        Rc::new(move || {
            if let Some(timeout) = timeout_ref.borrow_mut().take() {
                timeout.cancel();
            }
        })
    };

    // Cleanup on unmount
    use_unmount(move || {
        if let Some(timeout) = timeout_ref.borrow_mut().take() {
            timeout.cancel();
        }
    });

    UseDebounceHandle { run, cancel }
}
