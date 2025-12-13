use std::collections::VecDeque;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_queue` demo
#[function_component]
pub fn UseQueue() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onset}>{ "Set" }</Button>
                    <Button onclick={onpush_back}>{ "Push back" }</Button>
                    <Button onclick={onpop_front}>{ "Pop front" }</Button>
                    <Button onclick={onretain}>{ "Retain" }</Button>
                    <Button onclick={onclear}>{ "Clear all" }</Button>
                    <p>
                        <b>{ "Current queue: " }</b>
                    </p>
                    {
                        for queue.current().iter().map(|v| {
                            html! {
                                <p><b>{ *v }</b></p>
                            }
                        })
                    }
                </div>
            </header>
        </div>
    }
}
