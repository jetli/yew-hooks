use yew::prelude::*;

use super::use_is_first_mount;

/// This hook ignores the first invocation (e.g. on mount).
/// The signature is exactly the same as the [`use_effect`] hook.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseEffectUpdate)]
/// fn effect_update() -> Html {
///     use_effect_update(|| {
///         debug!("Running effect only on updates");
///     });
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_effect_update<Callback, Destructor>(callback: Callback)
where
    Callback: FnOnce() -> Destructor + 'static,
    Destructor: TearDown,
{
    let first = use_is_first_mount();

    use_effect(move || {
        if first {
            Box::new(|| ()) as Box<dyn FnOnce()>
        } else {
            let d = callback();
            Box::new(move || d.tear_down())
        }
    });
}

/// This hook is similar to [`use_effect_update`] but it accepts dependencies.
/// The signature is exactly the same as the [`use_effect_with`] hook.
#[hook]
pub fn use_effect_update_with_deps<Callback, Destructor, Dependents>(
    callback: Callback,
    deps: Dependents,
) where
    Callback: FnOnce(&Dependents) -> Destructor + 'static,
    Destructor: TearDown,
    Dependents: PartialEq + 'static,
{
    let first = use_is_first_mount();

    use_effect_with(deps, move |deps| {
        if first {
            Box::new(|| ()) as Box<dyn FnOnce()>
        } else {
            let d = callback(deps);
            Box::new(move || d.tear_down())
        }
    });
}
