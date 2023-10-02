use yew::prelude::*;

/// A lifecycle hook that runs an effect only once.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(EffectOnce)]
/// fn effect_once() -> Html {
///     use_effect_once(|| {
///         debug!("Running effect once on mount");
///         
///         || debug!("Running clean-up of effect on unmount")
///     });
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_effect_once<Callback, Destructor>(callback: Callback)
where
    Callback: FnOnce() -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
{
    use_effect_with((), move |_| callback());
}
