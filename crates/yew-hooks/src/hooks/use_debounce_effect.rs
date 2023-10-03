use yew::prelude::*;

use super::{use_debounce, use_unmount};

/// A hook that delays calling effect callback until after wait milliseconds have elapsed
/// since the last time effect callback was called.
///
/// # Example
///
/// ```rust
/// # use web_sys::HtmlInputElement;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(DebounceEffect)]
/// fn debounce_effect() -> Html {
///     let status = use_state(|| "Typing stopped".to_string());
///     let value = use_state(|| "".to_string());
///     let debounced_value = use_state(|| "".to_string());
///
///     {
///         let status = status.clone();
///         let value = value.clone();
///         let debounced_value = debounced_value.clone();
///         use_debounce_effect(
///             move || {
///                 // This will delay updating state.
///                 debounced_value.set((*value).clone());
///                 status.set("Typing stopped".to_string());
///             },
///             2000,
///         );
///     }
///
///     let oninput = {
///         let status = status.clone();
///         let value = value.clone();
///         Callback::from(move |e: InputEvent| {
///             let input: HtmlInputElement = e.target_unchecked_into();
///             // This will update state every time.
///             value.set(input.value());
///             status.set("Waiting for typing to stop...".to_string());
///         })
///     };
///
///     html! {
///         <>
///             <input type="text" value={(*value).clone()} placeholder="Debounced input" {oninput}/>
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
pub fn use_debounce_effect<Callback>(callback: Callback, millis: u32)
where
    Callback: FnOnce() + 'static,
{
    let debounce = use_debounce(callback, millis);

    {
        let debounce = debounce.clone();
        use_effect(move || {
            debounce.run();

            || ()
        });
    }

    use_unmount(move || {
        debounce.cancel();
    });
}

/// This hook is similar to [`use_debounce_effect`] but it accepts dependencies.
///
/// Whenever the dependencies are changed, the debounce effect is run again.
/// To detect changes, dependencies must implement `PartialEq`.
#[hook]
pub fn use_debounce_effect_with_deps<Callback, Dependents>(
    callback: Callback,
    millis: u32,
    deps: Dependents,
) where
    Callback: FnOnce() + 'static,
    Dependents: PartialEq + 'static,
{
    let debounce = use_debounce(callback, millis);

    {
        let debounce = debounce.clone();
        use_effect_with(deps, move |_| {
            debounce.run();

            || ()
        });
    }

    use_unmount(move || {
        debounce.cancel();
    });
}
