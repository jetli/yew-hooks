use std::rc::Rc;
use yew::use_mut_ref;

use super::use_effect_once;

/// A hook returns true if component is mounted and false otherwise.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// # use yew_hooks::use_is_mounted;
/// #
/// #[function_component(IsMounted)]
/// fn is_mounted() -> Html {
///     let is_mounted = use_is_mounted();
///     
///     html! {
///         <>
///             { is_mounted() }
///         </>
///     }
/// }
/// ```
pub fn use_is_mounted() -> Rc<impl Fn() -> bool> {
    let is_mounted = use_mut_ref(|| false);

    {
        let is_mounted = is_mounted.clone();

        use_effect_once(move || {
            *is_mounted.borrow_mut() = true;

            move || *is_mounted.borrow_mut() = false
        });
    }

    Rc::new(move || {
        let is_mounted = *is_mounted.borrow();
        is_mounted
    })
}
