use web_sys::Element;
use yew::prelude::*;

use super::{use_debounce, use_event, use_latest};

/// A sensor hook that tracks infinite scrolling of the element.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseInfiniteScroll)]
/// fn infinite_scroll() -> Html {
///     let node = use_node_ref();
///     let state = use_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
///
///     {
///         let state = state.clone();
///         use_infinite_scroll(node.clone(), move || {
///             let max = state.current().len() + 1;
///             let mut more = vec![max, max + 1, max + 2, max + 3, max + 4];
///             state.append(&mut more);
///         });
///     }
///
///     html! {
///         <div ref={node}>
///             {
///                 for state.current().iter().map(|element| {
///                     html! { <p>{ *element }</p> }
///                 })
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_infinite_scroll<Callback>(node: NodeRef, callback: Callback)
where
    Callback: Fn() + 'static,
{
    let callback_ref = use_latest(callback);
    let load_more = use_state_eq(|| false);

    {
        let load_more = load_more.clone();
        use_effect_with(load_more, move |load_more| {
            if **load_more {
                let callback = &*callback_ref.current();
                callback();
            }

            || ()
        });
    }

    let debounce = {
        let load_more = load_more.clone();
        use_debounce(
            move || {
                load_more.set(false);
            },
            100,
        )
    };

    use_event(node, "scroll", move |e: Event| {
        let element: Element = e.target_unchecked_into();
        if element.scroll_height() - element.scroll_top() <= element.client_height() + 100 {
            load_more.set(true);
            debounce.run();
        }
    });
}
