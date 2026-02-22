use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

/// State handle for the [`use_virtual_list`] hook.
#[derive(Clone, PartialEq)]
pub struct VirtualListItem<T> {
    /// The item data
    pub data: T,
    /// The index in the original list
    pub index: usize,
    /// The top position in pixels
    pub top: f64,
    /// The height of the item in pixels
    pub height: f64,
}

/// State handle for the [`use_virtual_list`] hook.
pub struct UseVirtualListHandle<T> {
    /// The visible items
    pub visible_items: Vec<VirtualListItem<T>>,
    /// The total height of all items
    pub total_height: f64,
    /// The start index of visible items
    pub start_index: usize,
    /// The end index of visible items
    pub end_index: usize,
    /// Function to scroll to a specific index
    pub scroll_to: Rc<dyn Fn(usize)>,
}

impl<T> Clone for UseVirtualListHandle<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            visible_items: self.visible_items.clone(),
            total_height: self.total_height,
            start_index: self.start_index,
            end_index: self.end_index,
            scroll_to: self.scroll_to.clone(),
        }
    }
}

impl<T> PartialEq for UseVirtualListHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.visible_items == other.visible_items
            && self.total_height == other.total_height
            && self.start_index == other.start_index
            && self.end_index == other.end_index
    }
}

/// A hook that provides virtual scrolling for large lists.
///
/// This hook calculates which items should be visible based on the scroll position
/// and container height, improving performance for large lists.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(VirtualList)]
/// fn virtual_list() -> Html {
///     let items = (0..10000).collect::<Vec<_>>();
///     let item_height = |index: usize| 50.0;
///     let container = use_node_ref();
///     let wrapper = use_node_ref();
///     let overscan = 5;
///
///     let virtual_list = use_virtual_list(
///         items,
///         item_height,
///         container.clone(),
///         wrapper.clone(),
///         overscan,
///     );
///
///     html! {
///         <>
///             <div
///                 ref={container}
///                 style="height: 400px; overflow: auto;"
///             >
///                 <div ref={wrapper}>
///                     {
///                         for virtual_list.visible_items.iter().map(|item| {
///                             html! {
///                                 <div
///                                     key={item.index}
///                                     style={format!(
///                                         "position: absolute; top: {}px; height: {}px; width: 100%;",
///                                         item.top, item.height
///                                     )}
///                                 >
///                                     { format!("Item {}", item.data) }
///                                 </div>
///                             }
///                         })
///                     }
///                 </div>
///             </div>
///             <button onclick={let scroll_to = virtual_list.scroll_to.clone(); Callback::from(move |_| scroll_to(100))}>{"Scroll to 100"}</button>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_virtual_list<T>(
    items: Vec<T>,
    item_height: fn(usize) -> f64,
    container: NodeRef,
    wrapper: NodeRef,
    overscan: usize,
) -> UseVirtualListHandle<T>
where
    T: Clone + PartialEq + 'static,
{
    let scroll_position = use_state(|| 0.0);
    let container_height = use_state(|| 0.0);
    let handle = use_state(|| UseVirtualListHandle {
        visible_items: vec![],
        total_height: 0.0,
        start_index: 0,
        end_index: 0,
        scroll_to: Rc::new(|_| {}),
    });

    {
        let items = items.clone();
        let scroll_top_val = *scroll_position;
        let container_height_val = *container_height;
        let handle_clone = handle.clone();
        let wrapper_clone = wrapper.clone();
        let scroll_position_clone = scroll_position.clone();
        let container_clone = container.clone();
        use_effect_with(
            (items, container_height_val, scroll_top_val, overscan),
            move |(items, container_height, scroll_top, overscan)| {
                let heights: Vec<f64> = (0..items.len()).map(item_height).collect();
                let total_height = heights.iter().sum::<f64>();
                let mut cumulative = 0.0;
                let mut start_index = 0;
                for (i, &h) in heights.iter().enumerate() {
                    if cumulative + h > *scroll_top {
                        start_index = i;
                        break;
                    }
                    cumulative += h;
                }
                let start_cum = cumulative;
                let mut end_index = start_index;
                let mut current_cum = start_cum;
                while current_cum < *scroll_top + *container_height && end_index < items.len() {
                    current_cum += heights[end_index];
                    end_index += 1;
                }
                end_index = end_index.min(items.len());
                let visible_start = start_index;
                let visible_end = end_index;
                let render_start = visible_start.saturating_sub(*overscan);
                let render_end = (visible_end + *overscan).min(items.len());
                let visible_items = (render_start..render_end)
                    .map(|index| {
                        let top = heights[0..index].iter().sum::<f64>();
                        VirtualListItem {
                            data: items[index].clone(),
                            index,
                            top,
                            height: heights[index],
                        }
                    })
                    .collect();
                let scroll_to = {
                    let heights = heights.clone();
                    let st_setter = scroll_position_clone.clone();
                    let container = container_clone.clone();
                    Rc::new(move |index: usize| {
                        if index < heights.len() {
                            let top = heights[0..index].iter().sum::<f64>();
                            st_setter.set(top);
                            if let Some(c) = container.get() {
                                if let Some(e) = c.dyn_ref::<web_sys::HtmlElement>() {
                                    e.set_scroll_top(top as _);
                                }
                            }
                        }
                    })
                };
                let new_handle = UseVirtualListHandle {
                    visible_items,
                    total_height,
                    start_index: visible_start,
                    end_index: visible_end.saturating_sub(1),
                    scroll_to,
                };
                handle_clone.set(new_handle.clone());
                // Set height on wrapper
                if let Some(w) = wrapper_clone.get() {
                    if let Some(e) = w.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = e
                            .style()
                            .set_property("height", &format!("{}px", total_height));
                        let _ = e.style().set_property("position", "relative");
                    }
                }
            },
        );
    }

    {
        let container_clone = container.clone();
        let scroll_position_clone = scroll_position.clone();
        let container_height_clone = container_height.clone();
        use_effect_with(container_clone, move |container| {
            if let Some(c) = container.get() {
                let c = c.clone();
                if let Some(e) = c.dyn_ref::<web_sys::HtmlElement>() {
                    #[allow(clippy::unnecessary_cast)]
                    container_height_clone.set(e.client_height() as f64);
                    #[allow(clippy::unnecessary_cast)]
                    scroll_position_clone.set(e.scroll_top() as f64);
                    let scroll_top_inner = scroll_position_clone.clone();
                    let c_clone = c.clone();
                    let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        if let Some(e) = c_clone.dyn_ref::<web_sys::HtmlElement>() {
                            #[allow(clippy::unnecessary_cast)]
                            scroll_top_inner.set(e.scroll_top() as f64);
                        }
                    }) as Box<dyn FnMut(_)>);
                    let _ = e.add_event_listener_with_callback(
                        "scroll",
                        closure.as_ref().unchecked_ref(),
                    );
                    closure.forget();
                }
            }
            || {}
        });
    }

    (*handle).clone()
}
