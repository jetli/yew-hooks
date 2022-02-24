use yew::prelude::*;

use yew_hooks::{use_drop, use_drop_with_options, UseDropOptions};

/// `use_drop` demo
#[function_component(UseDrop)]
pub fn drop() -> Html {
    let node = use_node_ref();
    // Demo #1, simply without callback options.
    let state = use_drop(node.clone());

    // Demo #2, use callback options.
    let _ = use_drop_with_options(
        node.clone(),
        UseDropOptions {
            onfiles: Some(Box::new(move |_files, _data_transfer| {
                // Process files or data_transfer
            })),
            ..Default::default()
        },
    );

    html! {
        <div class="app">
            <header class="app-header">
                <div ref={node} style={if *state.over {
                        "background-color: #71eaff; border: 5px dashed white;"
                    } else {
                        "background-color: #61dafb; border: 3px dashed white" }}>
                    <p><b>{ " Files: " }</b></p>
                    {if let Some(files) = &*state.files {
                        html! {for files.iter().map(|file| {
                            html! { <p> { file.name() }</p> }
                        })}
                    } else {
                        html! {}
                    }}
                    <p><b>{ " Text: " }</b></p>
                    {if let Some(text) = &*state.text {
                        html! {<p>{ text }</p>}
                    } else {
                        html! {}
                    }}
                    <p><b>{ " Uri: " }</b></p>
                    {if let Some(uri) = &*state.uri {
                        html! {<p>{ uri }</p>}
                    } else {
                        html! {}
                    }}
                    <p>
                        { "Try to drag & drop or copy & paste something here, e.g. files, links or text" }
                    </p>
                </div>
            </header>
        </div>
    }
}
