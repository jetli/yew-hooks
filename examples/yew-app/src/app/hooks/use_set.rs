use std::collections::HashSet;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_set` demo
#[function_component]
pub fn UseSet() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onset}>{ "Set" }</Button>
                    <Button onclick={oninsert}>{ "Insert" }</Button>
                    <Button onclick={onreplace}>{ "Replace" }</Button>
                    <Button onclick={onremove}>{ "Remove" }</Button>
                    <Button onclick={onretain}>{ "Retain" }</Button>
                    <Button onclick={onclear}>{ "Clear all" }</Button>
                    <p>
                        <b>{ "Current set: " }</b>
                    </p>
                    {
                        for set.current().iter().map(|v| {
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
