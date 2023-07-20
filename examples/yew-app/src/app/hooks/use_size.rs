use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_size` demo
#[function_component]
pub fn UseSize() -> Html {
    let node = use_node_ref();
    let state = use_size(node.clone());

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div ref={node} class="bg-emerald-800 text-slate-100">
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
