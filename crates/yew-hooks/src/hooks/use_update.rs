use std::rc::Rc;
use yew::use_state;

/// A hook returns a function that forces component to re-render when called.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// # use yew_hooks::use_update;
/// #
/// #[function_component(Update)]
/// fn update() -> Html {
///     let update = use_update();
///
///     let onclick = Callback::from(move |_| {
///         update();
///     });
///     
///     html! {
///         <>
///             <button {onclick}>{ "Update" }</button>
///         </>
///     }
/// }
/// ```
pub fn use_update() -> Rc<impl Fn()> {
    let state = use_state(|| 0);

    Rc::new(move || {
        state.set((*state + 1) % 1_000_000);
    })
}
