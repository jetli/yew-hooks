use std::collections::HashMap;

use yew::prelude::*;

use yew_hooks::use_map;

/// `use_map` demo
#[function_component(UseMap)]
pub fn map() -> Html {
    let map = use_map(HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]));

    let onset = {
        let map = map.clone();
        Callback::from(move |_| map.set(HashMap::from([("Moon", 0.8), ("Earth", 1.0)])))
    };
    let oninsert = {
        let map = map.clone();
        Callback::from(move |_| {
            let _ = map.insert("Jupiter", 2.1);
        })
    };
    let onupdate = {
        let map = map.clone();
        Callback::from(move |_| map.update(&"Earth", 1.1))
    };
    let onremove = {
        let map = map.clone();
        Callback::from(move |_| {
            let _ = map.remove(&"Moon");
        })
    };
    let onretain = {
        let map = map.clone();
        Callback::from(move |_| map.retain(|_k, v| v > &mut 1.0))
    };
    let onclear = {
        let map = map.clone();
        Callback::from(move |_| map.clear())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onset}>{ "Set" }</button>
                    <button onclick={oninsert}>{ "Insert" }</button>
                    <button onclick={onupdate}>{ "Update" }</button>
                    <button onclick={onremove}>{ "Remove" }</button>
                    <button onclick={onretain}>{ "Retain" }</button>
                    <button onclick={onclear}>{ "Clear all" }</button>
                    <p>
                        <b>{ "Current map: " }</b>
                    </p>
                    {
                        for map.current().iter().map(|(k, v)| {
                            html! {
                                <p><b>{ k }</b> {": "} { v }</p>
                            }
                        })
                    }
                </div>
            </header>
        </div>
    }
}
