use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_infinite_scroll` demo
#[function_component]
pub fn UseInfiniteScroll() -> Html {
    let node = use_node_ref();
    let state = use_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    {
        let state = state.clone();
        use_infinite_scroll(node.clone(), move || {
            let max = state.current().len() + 1;
            let mut more = vec![max, max + 1, max + 2, max + 3, max + 4, max + 5, max + 6];
            state.append(&mut more);
        });
    }

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <div ref={node} class="w-[600px] h-[300px] overflow-scroll bg-emerald-800 mx-auto">
                        <div>
                            { "Try to scroll in this area vertically." }
                            {
                                for state.current().iter().map(|element| {
                                    html! { <p style="height: 50px;">{ *element }</p> }
                                })
                            }

                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
