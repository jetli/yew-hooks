use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_drag` demo
#[function_component]
pub fn UseDragComponent() -> Html {
    let node = use_node_ref();
    // Demo #1, simply without callback options.
    let state = use_drag(node.clone());

    // Demo #2, use callback options.
    let _ = use_drag_with_options(
        node.clone(),
        UseDragOptions {
            ondragstart: Some(Box::new(move |e| {
                if let Some(data_transfer) = e.data_transfer() {
                    // You can send some text via DataTransfer to the dropped target.
                    let _ = data_transfer.set_data("text", "hello from draggable component");
                }
            })),
            ..Default::default()
        },
    );

    html! {
        <div ref={node} class={if *state.dragging {
                "bg-emerald-800 border-4 border-dashed border-white"
            } else {
                "bg-emerald-600 border-4 border-dashed border-white" }}>
            <p>
                <b>{ " Dagging: " }</b>
                { *state.dragging }
            </p>
            <p>
                { "Try to drag this area" }
            </p>
        </div>
    }
}

#[function_component]
pub fn UseDrag() -> Html {
    let node = use_node_ref();
    let state = use_drop(node.clone());

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <UseDragComponent />

                <div ref={node} class={if *state.over {
                        "bg-emerald-800 border-4 border-dashed border-white mt-8"
                    } else {
                        "bg-emerald-600 border-4 border-dashed border-white mt-8" }}>
                    <p><b>{ " Text: " }</b></p>
                        {
                            (*state.text).as_ref().map_or_else(|| html! {},
                                                               |text| html! {<p>{ text }</p>})
                        }
                    <p>
                        { "Try to drop something to this area" }
                    </p>
                </div>
            </header>
        </div>
    }
}
