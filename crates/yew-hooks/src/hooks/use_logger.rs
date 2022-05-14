use std::fmt::Debug;
use std::rc::Rc;

use yew::prelude::*;

use super::{use_effect_once, use_effect_update, use_effect_update_with_deps, use_previous};

/// This hook logs in console as component goes through life-cycles.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseLogger)]
/// fn logger(props: &Props) -> Html {
///     use_logger("MyComponent".to_string(), props.clone());
///
///     let c = use_state(|| 0);
///     let d = use_state(|| "d".to_string());
///     use_logger("MyComponent".to_string(), (c.clone(), d.clone()));
///     
///     html! {
///         <>
///             <b>{ " a: " }</b> { props.a }
///             <b>{ " b: " }</b> { &props.b }
///             <b>{ " c: " }</b> { *c }
///             <b>{ " d: " }</b> { &*d }
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
#[hook]
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

/// This hook logs in console as component goes through life-cycles.
/// Like [`use_logger`] but only logs when `prev_state != next_state`.
/// This requires the props to implement [`PartialEq`].
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseLogger)]
/// fn logger(props: &Props) -> Html {
///     use_logger_eq("MyComponent".to_string(), props.clone());
///
///     let c = use_state(|| 0);
///     let d = use_state(|| "d".to_string());
///     use_logger_eq("MyComponent".to_string(), (c.clone(), d.clone()));
///     
///     html! {
///         <>
///             <b>{ " a: " }</b> { props.a }
///             <b>{ " b: " }</b> { &props.b }
///             <b>{ " c: " }</b> { *c }
///             <b>{ " d: " }</b> { &*d }
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
#[hook]
pub fn use_logger_eq<T>(name: String, props: T)
where
    T: Debug + PartialEq + 'static,
{
    let name = Rc::new(name);
    let props = Rc::new(props);
    let props_prev = use_previous(props.clone());

    {
        let name = name.clone();
        let props = props.clone();

        use_effect_once(move || {
            log::debug!("{} mounted: {:?}", name, props);

            move || log::debug!("{} unmounted", name)
        });
    }

    use_effect_update_with_deps(
        move |props| {
            log::debug!("{} updated: from {:?}, to {:?}", name, *props_prev, props);
            || ()
        },
        props,
    );
}
