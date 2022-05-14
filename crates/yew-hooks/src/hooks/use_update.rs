use std::rc::Rc;

use yew::prelude::*;

/// A hook returns a function that forces component to re-render when called.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
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
#[hook]
pub fn use_update() -> Rc<dyn Fn()> {
    let state = use_state(|| 0);

    Rc::new(move || {
        state.set((*state + 1) % 1_000_000);
    })
}
