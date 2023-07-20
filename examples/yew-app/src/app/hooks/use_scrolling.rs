use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_scrolling` demo
#[function_component]
pub fn UseScrolling() -> Html {
    let node = use_node_ref();
    let state = use_scrolling(node.clone());

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ " Scrolling: " }</b>
                        { if state { "Scrolling" } else { "Not scrolling" } }
                    </p>
                    <div ref={node} class="w-[600px] h-[400px] overflow-scroll bg-emerald-800 mx-auto text-slate-100">
                        <div class="w-[1000px] h-[1000px] text-left">
                            { "Try to scroll in this area vertically or horizontally." }
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
