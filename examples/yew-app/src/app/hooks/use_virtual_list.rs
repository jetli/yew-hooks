use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_virtual_list` demo
#[function_component]
pub fn UseVirtualList() -> Html {
    let items = (0..10000).collect::<Vec<_>>();
    let item_height = |_index: usize| 50.0;
    let items_len = items.len();
    let container = use_node_ref();
    let wrapper = use_node_ref();
    let overscan = 5;

    let virtual_list = use_virtual_list(
        items,
        item_height,
        container.clone(),
        wrapper.clone(),
        overscan,
    );

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>{ format!("Total items: {}, Visible: {}-{}", items_len, virtual_list.start_index, virtual_list.end_index) }</p>
                    <Button
                        onclick={let scroll_to = virtual_list.scroll_to.clone(); Callback::from(move |_| scroll_to(100))}
                    >
                        { "Scroll to 100" }
                    </Button>
                    <div
                        ref={container}
                        style="height: 400px; overflow: auto; border: 1px solid #ccc;"
                    >
                        <div ref={wrapper}>
                            {
                                for virtual_list.visible_items.iter().map(|item| {
                                    html! {
                                        <div
                                            key={item.index}
                                            style={format!(
                                                "position: absolute; top: {}px; height: {}px; width: 100%; border-bottom: 1px solid #eee; padding: 10px; box-sizing: border-box;",
                                                item.top, item.height
                                            )}
                                        >
                                            { format!("Item {} (index: {})", item.data, item.index) }
                                        </div>
                                    }
                                })
                            }
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
