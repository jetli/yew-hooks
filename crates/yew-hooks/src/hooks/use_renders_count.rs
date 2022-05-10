use yew::prelude::*;

/// A hook that counts component renders.
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
///     let count = use_renders_count();
///     let update = use_update();
///
///     let onclick = Callback::from(move |_| {
///         update();
///     });
///     
///     html! {
///         <>
///             <button {onclick}>{ "Update" }</button>
///             { count }
///         </>
///     }
/// }
/// ```
pub fn use_renders_count() -> i32 {
    let count = use_mut_ref(|| 0);

    let current = *count.borrow();
    *count.borrow_mut() = current + 1;
    let current = *count.borrow();

    current
}
