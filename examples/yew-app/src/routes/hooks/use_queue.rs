use std::collections::VecDeque;

use yew::prelude::*;

use yew_hooks::use_queue;

/// `use_queue` demo
#[function_component(UseQueue)]
pub fn queue() -> Html {
    let queue = use_queue(VecDeque::from(["Mercury", "Venus", "Earth", "Mars"]));

    let onset = {
        let queue = queue.clone();
        Callback::from(move |_| queue.set(VecDeque::from(["Moon", "Earth"])))
    };
    let onpush_back = {
        let queue = queue.clone();
        Callback::from(move |_| {
            queue.push_back("Jupiter");
        })
    };
    let onpop_front = {
        let queue = queue.clone();
        Callback::from(move |_| {
            let _ = queue.pop_front();
        })
    };
    let onretain = {
        let queue = queue.clone();
        Callback::from(move |_| queue.retain(|v| v.contains('a')))
    };
    let onclear = {
        let queue = queue.clone();
        Callback::from(move |_| queue.clear())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onset}>{ "Set" }</button>
                    <button onclick={onpush_back}>{ "Push back" }</button>
                    <button onclick={onpop_front}>{ "Pop front" }</button>
                    <button onclick={onretain}>{ "Retain" }</button>
                    <button onclick={onclear}>{ "Clear all" }</button>
                    <p>
                        <b>{ "Current queue: " }</b>
                    </p>
                    {
                        for queue.current().iter().map(|v| {
                            html! {
                                <p><b>{ v }</b></p>
                            }
                        })
                    }
                </div>
            </header>
        </div>
    }
}
