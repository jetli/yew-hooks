use yew::prelude::*;

use yew_hooks::{use_drag, use_drag_with_options, use_drop, UseDragOptions};

/// `use_drag` demo
#[function_component(UseDragComponent)]
pub fn drag_component() -> Html {
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
        <div ref={node} style={if *state.dragging {
                "background-color: #71eaff; border: 5px dashed white;"
            } else {
                "background-color: #61dafb; border: 3px dashed white" }}>
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

#[function_component(UseDrag)]
pub fn drag() -> Html {
    let node = use_node_ref();
    let state = use_drop(node.clone());

    html! {
        <div class="app">
            <header class="app-header">
                <UseDragComponent />

                <div ref={node} style={if *state.over {
                        "background-color: #71eaff; border: 5px dashed white; margin-top: 20px;"
                    } else {
                        "background-color: #61dafb; border: 3px dashed white; margin-top: 20px;" }}>
                    <p><b>{ " Text: " }</b></p>
                        {if let Some(text) = &*state.text {
                            html! {<p>{ text }</p>}
                        } else {
                            html! {}
                        }}
                    <p>
                        { "Try to drop something to this area" }
                    </p>
                </div>
            </header>
        </div>
    }
}
