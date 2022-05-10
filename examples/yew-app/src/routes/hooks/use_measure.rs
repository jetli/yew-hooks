use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_measure` demo
#[function_component(UseMeasure)]
pub fn measure() -> Html {
    let node = use_node_ref();
    let state = use_measure(node.clone());

    html! {
        <div class="app">
            <header class="app-header">
                <div ref={node} style="background-color: #61dafb; width: 100%;">
                    <p>
                        <b>{ " X: " }</b>
                        { state.x }
                        <b>{ " Y: " }</b>
                        { state.y }
                        <b>{ " Width: " }</b>
                        { state.width }
                        <b>{ " Height: " }</b>
                        { state.height }
                        <b>{ " Top: " }</b>
                        { state.top }
                        <b>{ " Left: " }</b>
                        { state.left }
                        <b>{ " Bottom: " }</b>
                        { state.bottom }
                        <b>{ " Right: " }</b>
                        { state.right }
                    </p>
                    <p>
                        { "Try to resize the window of your browser." }
                    </p>
                </div>
            </header>
        </div>
    }
}
