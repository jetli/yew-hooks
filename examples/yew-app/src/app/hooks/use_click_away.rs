use gloo::dialogs::alert;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_click_away` demo
#[function_component]
pub fn UseClickAway() -> Html {
    let node = use_node_ref();
    use_click_away(node.clone(), move |_: Event| {
        alert("Clicked outside!");
    });

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div ref={node} class="bg-emerald-800 w-[400px] h-[200px] mx-auto text-slate-100 p-4">
                    <p>
                        { "Try to click outside of this area." }
                    </p>
                </div>
            </header>
        </div>
    }
}
