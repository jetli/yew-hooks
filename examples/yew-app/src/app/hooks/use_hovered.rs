use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_hovered` demo
#[function_component]
pub fn UseHovered() -> Html {
    let node = use_node_ref();
    let hovered = use_hovered(node.clone());

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div ref={node} class="bg-emerald-800 w-[400px] h-[200px] mx-auto text-slate-100 p-4">
                    <p>
                        <b>{ " Hovered: " }</b>
                        { hovered }
                    </p>
                    <p>
                        { "Try to hover your cursor over this element." }
                    </p>
                </div>
            </header>
        </div>
    }
}
