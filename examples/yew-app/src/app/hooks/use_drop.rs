use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_drop` demo
#[function_component]
pub fn UseDrop() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div ref={node} class={if *state.over {
                        "bg-emerald-800 border-4 border-dashed border-white mt-8"
                    } else {
                        "bg-emerald-600 border-4 border-dashed border-white mt-8" }}>
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
