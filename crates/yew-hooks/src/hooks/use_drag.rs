use web_sys::Element;
use yew::prelude::*;

use super::{use_event, use_mut_latest};

/// Options for drag.
#[derive(Default)]
pub struct UseDragOptions {
    /// Callback for `dragstart`.
    pub ondragstart: Option<Box<dyn FnMut(DragEvent)>>,
    /// Callback for `dragend`.
    pub ondragend: Option<Box<dyn FnMut(DragEvent)>>,
}

/// State handle for the [`use_drag`] hook.
pub struct UseDragHandle {
    /// State for whether is dragging.
    pub dragging: UseStateHandle<bool>,
}

/// This hook tracks file, link and copy-paste drags.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseDrag)]
/// fn drag() -> Html {
///     let node = use_node_ref();
///     let state = use_drag(node.clone());
///
///     html! {
///         <div ref={node}>
///             <p>
///                 <b>{ " Dragging: " }</b>
///                 { *state.dragging }
///             </p>
///             <p>
///                 { "Try to drag this area" }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_drag(node: NodeRef) -> UseDragHandle {
    use_drag_with_options(node, UseDragOptions::default())
}

/// This hook tracks file, link and copy-paste drags.
/// [`use_drag`] hook with options.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseDrag)]
/// fn drag() -> Html {
///     let node = use_node_ref();
///     let state = use_drag_with_options(node.clone(), UseDragOptions {
///         ondragstart: Some(Box::new(move |e| {
///             if let Some(data_transfer) = e.data_transfer() {
///                 let _ = data_transfer.set_data("text", "hello");
///             }
///         })),
///         ondragend: Some(Box::new(move |e| {
///         })),
///     });
///
///     html! {
///         <div ref={node}>
///             <p>
///                 <b>{ " Dragging: " }</b>
///                 { *state.dragging }
///             </p>
///             <p>
///                 { "Try to drag this area" }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_drag_with_options(node: NodeRef, options: UseDragOptions) -> UseDragHandle {
    let dragging = use_state(|| false);

    let ondragstart_ref = use_mut_latest(options.ondragstart);
    let ondragend_ref = use_mut_latest(options.ondragend);

    {
        let dragging = dragging.clone();
        use_event(node.clone(), "dragstart", move |e: DragEvent| {
            dragging.set(true);
            let ondragstart_ref = ondragstart_ref.current();
            let ondragstart = &mut *ondragstart_ref.borrow_mut();
            if let Some(ondragstart) = ondragstart {
                ondragstart(e);
            }
        });
    }

    {
        let dragging = dragging.clone();
        use_event(node.clone(), "dragend", move |e: DragEvent| {
            dragging.set(false);
            let ondragend_ref = ondragend_ref.current();
            let ondragend = &mut *ondragend_ref.borrow_mut();
            if let Some(ondragend) = ondragend {
                ondragend(e);
            }
        });
    }

    use_effect_with_deps(
        move |node| {
            if let Some(element) = &node.cast::<Element>() {
                let _ = element.set_attribute("draggable", "true");
            }

            || ()
        },
        node,
    );

    UseDragHandle { dragging }
}
