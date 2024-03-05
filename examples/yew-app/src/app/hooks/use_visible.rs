use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_visible` demo
#[function_component]
pub fn UseVisible() -> Html {
    let node = use_node_ref();
    let visible = use_visible(node.clone(), false);

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ " Visible: " }</b>
                        { visible }
                    </p>
                    <div class="w-[600px] h-[400px] overflow-scroll bg-emerald-800 mx-auto text-slate-100">
                        <div class="w-[1000px] h-[1000px] text-left">
                            <div class="h-[600px]">{ "Try to scroll in this area." }</div>
                            <div ref={node} class="w-[100px] h-[100px] bg-slate-800"></div>
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
