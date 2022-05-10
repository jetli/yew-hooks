use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_size` demo
#[function_component(UseSize)]
pub fn size() -> Html {
    let node = use_node_ref();
    let state = use_size(node.clone());

    html! {
        <div class="app">
            <header class="app-header">
                <div ref={node} style="background-color: #61dafb; width: 100%;">
                    <p>
                        <b>{ " Width: " }</b>
                        { state.0 }
                        <b>{ " Height: " }</b>
                        { state.1 }
                    </p>
                    <p>
                        { "Try to resize the window of your browser." }
                    </p>
                </div>
            </header>
        </div>
    }
}
