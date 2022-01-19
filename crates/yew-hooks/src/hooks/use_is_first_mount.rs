use yew::prelude::*;

/// A hook returns true if component is just mounted (on first render) and false otherwise.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// # use yew_hooks::use_is_first_mount;
/// #
/// #[function_component(IsFirstMount)]
/// fn is_first_mount() -> Html {
///     let is_first = use_is_first_mount();
///     
///     html! {
///         <>
///             { is_first }
///         </>
///     }
/// }
/// ```
pub fn use_is_first_mount() -> bool {
    let is_first = use_mut_ref(|| true);

    if *is_first.borrow_mut() {
        *is_first.borrow_mut() = false;

        return true;
    }

    false
}
