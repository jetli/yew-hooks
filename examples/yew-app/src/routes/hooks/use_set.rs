use std::collections::HashSet;

use yew::prelude::*;

use yew_hooks::use_set;

/// `use_set` demo
#[function_component(UseSet)]
pub fn set() -> Html {
    let set = use_set(HashSet::from(["Mercury", "Venus", "Earth", "Mars"]));

    let onset = {
        let set = set.clone();
        Callback::from(move |_| set.set(HashSet::from(["Moon", "Earth"])))
    };
    let oninsert = {
        let set = set.clone();
        Callback::from(move |_| {
            let _ = set.insert("Jupiter");
        })
    };
    let onreplace = {
        let set = set.clone();
        Callback::from(move |_| {
            let _ = set.replace("Earth");
        })
    };
    let onremove = {
        let set = set.clone();
        Callback::from(move |_| {
            let _ = set.remove(&"Moon");
        })
    };
    let onretain = {
        let set = set.clone();
        Callback::from(move |_| set.retain(|v| v.contains('a')))
    };
    let onclear = {
        let set = set.clone();
        Callback::from(move |_| set.clear())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onset}>{ "Set" }</button>
                    <button onclick={oninsert}>{ "Insert" }</button>
                    <button onclick={onreplace}>{ "Replace" }</button>
                    <button onclick={onremove}>{ "Remove" }</button>
                    <button onclick={onretain}>{ "Retain" }</button>
                    <button onclick={onclear}>{ "Clear all" }</button>
                    <p>
                        <b>{ "Current set: " }</b>
                    </p>
                    {
                        for set.current().iter().map(|v| {
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
