use yew::prelude::*;

use super::{use_throttle, use_unmount};

/// A hook that throttles calling effect callback, it is only called once every `millis`.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(ThrottleEffect)]
/// fn throttle_effect() -> Html {
///     let state = use_state(|| 0);
///     let update = use_update();
///
///     {
///         let state = state.clone();
///         use_throttle_effect(
///             move || {
///                 state.set(*state + 1);
///             },
///             2000,
///         )
///     };
///
///     let onclick = { Callback::from(move |_| update()) };
///
///     html! {
///         <>
///             <button {onclick}>{ "Click fast!" }</button>
///             <b>{ "State: " }</b> {*state}
///         </>
///     }
/// }
/// ```
pub fn use_throttle_effect<Callback>(callback: Callback, millis: u32)
where
    Callback: FnMut() + 'static,
{
    let throttle = use_throttle(callback, millis);

    {
        let throttle = throttle.clone();
        use_effect(move || {
            throttle.run();

            || ()
        });
    }

    use_unmount(move || {
        throttle.cancel();
    });
}

/// This hook is similar to [`use_throttle_effect`] but it accepts dependencies.
///
/// Whenever the dependencies are changed, the throttle effect is run again.
/// To detect changes, dependencies must implement `PartialEq`.
pub fn use_throttle_effect_with_deps<Callback, Dependents>(
    callback: Callback,
    millis: u32,
    deps: Dependents,
) where
    Callback: FnMut() + 'static,
    Dependents: PartialEq + 'static,
{
    let throttle = use_throttle(callback, millis);

    {
        let throttle = throttle.clone();
        use_effect_with_deps(
            move |_| {
                throttle.run();

                || ()
            },
            deps,
        );
    }

    use_unmount(move || {
        throttle.cancel();
    });
}
