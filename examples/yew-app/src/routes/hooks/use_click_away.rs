use gloo::dialogs::alert;
use yew::prelude::*;

use yew_hooks::use_click_away;

/// `use_click_away` demo
#[function_component(UseClickAway)]
pub fn click_away() -> Html {
    let node = use_node_ref();
    use_click_away(node.clone(), move |_: Event| {
        alert("Clicked outside!");
    });

    html! {
        <div class="app">
            <header class="app-header">
                <div ref={node} style="background-color: #61dafb; width: 200px; height: 200px;">
                    <p>
                        { "Try to click outside of this area." }
                    </p>
                </div>
            </header>
        </div>
    }
}
