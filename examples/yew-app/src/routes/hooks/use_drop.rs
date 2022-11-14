use yew::prelude::*;
use yew_hooks::prelude::*;

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
                        "background-color: #61dafb; border: 3px dashed white;" }}>
                    <p><b>{ " Files: " }</b></p>
                    {
                        (*state.files).as_ref().map_or_else(
                            || html! {},
                            |files| {
                                html! {for files.iter().map(|file| {
                                    html! { <p> { file.name() }</p> }
                                })}
                            },
                        )
                    }
                    <p><b>{ " Text: " }</b></p>
                    {
                        (*state.text)
                            .as_ref()
                            .map_or_else(|| html! {}, |text| html! {<p>{ text }</p>})
                    }
                    <p><b>{ " Uri: " }</b></p>
                    {
                        (*state.uri)
                            .as_ref()
                            .map_or_else(|| html! {}, |uri| html! {<p>{ uri }</p>})
                    }
                    <p>
                        { "Try to drag & drop or copy & paste something here, e.g. files, links or text" }
                    </p>
                </div>
            </header>
        </div>
    }
}
