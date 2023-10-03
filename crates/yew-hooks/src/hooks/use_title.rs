use yew::prelude::*;

use super::use_unmount;

/// A side-effect hook that sets title of the page and restore previous title when unmount.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Title)]
/// fn title() -> Html {
///     use_title("This is an awesome title".to_string());
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_title(title: String) {
    let pre_title = use_memo((), |_| gloo::utils::document().title());

    if gloo::utils::document().title() != title {
        gloo::utils::document().set_title(&title);
    }

    use_unmount(move || {
        gloo::utils::document().set_title(&pre_title);
    });
}
