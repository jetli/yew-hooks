use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_infinite_scroll` demo
#[function_component(UseInfiniteScroll)]
pub fn infinite_scroll() -> Html {
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
        <div class="app">
            <header class="app-header">
                <div>
                    <div ref={node} style="width: 600px; height:300px; overflow: scroll; background-color: #61dafb;">
                        <div>
                            { "Try to scroll in this area vertically." }
                            {
                                for state.current().iter().map(|element| {
                                    html! { <p style="height: 50px;">{ element }</p> }
                                })
                            }

                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
