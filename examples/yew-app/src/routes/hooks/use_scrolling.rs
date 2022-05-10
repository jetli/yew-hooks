use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_scrolling` demo
#[function_component(UseScrolling)]
pub fn scroll() -> Html {
    let node = use_node_ref();
    let state = use_scrolling(node.clone());

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ " Scrolling: " }</b>
                        { if state { "Scrolling" } else { "Not scrolling" } }
                    </p>
                    <div ref={node} style="width: 600px; height:400px; overflow: scroll; background-color: #61dafb;">
                        <div style="width: 1000px; height: 1000px; text-align: left;">
                            { "Try to scroll in this area vertically or horizontally." }
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
