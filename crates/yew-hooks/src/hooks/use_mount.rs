use super::use_effect_once;

/// A lifecycle hook that calls a function after the component is mounted.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// # use yew_hooks::use_mount;
/// #
/// #[function_component(Mount)]
/// fn mount() -> Html {
///     use_mount(|| {
///         debug!("Running effect once on mount");
///     });
///     
///     html! {
///         <>
///         </>
///     }
///
/// }
/// ```
pub fn use_mount<Callback>(callback: Callback)
where
    Callback: FnOnce() + 'static,
{
    use_effect_once(move || {
        callback();

        || ()
    });
}
