use std::fmt::Debug;
use std::rc::Rc;

use super::{use_effect_once, use_effect_update};

/// This hook logs in console as component goes through life-cycles.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_logger;
///
/// #[function_component(UseLogger)]
/// fn logger(props: &Props) -> Html {
///     use_logger("MyComponent".to_string(), props.clone());
///
///     let c = use_state(|| 0);
///     let d = use_state(|| "d".to_string());
///     use_logger("MyComponent".to_string(), (*c, (*d).clone()));
///     
///     html! {
///         <>
///             <b>{ " a: " }</b> { props.a }
///             <b>{ " b: " }</b> { &props.b }
///         </>
///     }
/// }
///
/// #[derive(Debug, Properties, PartialEq, Clone)]
/// struct Props {
///     pub a: i32,
///     pub b: String,
/// }
/// ```
pub fn use_logger<T>(name: String, props: T)
where
    T: Debug + 'static,
{
    let name = Rc::new(name);
    let props = Rc::new(props);

    {
        let name = name.clone();
        let props = props.clone();

        use_effect_once(move || {
            log::debug!("{} mounted: {:?}", name, props);

            move || log::debug!("{} unmounted", name)
        });
    }

    use_effect_update(move || {
        log::debug!("{} updated: {:?}", name, props);
        || ()
    });
}
