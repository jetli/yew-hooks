use std::collections::HashMap;

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_map` demo
#[function_component]
pub fn UseMap() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onset}>{ "Set" }</Button>
                    <Button onclick={oninsert}>{ "Insert" }</Button>
                    <Button onclick={onupdate}>{ "Update" }</Button>
                    <Button onclick={onremove}>{ "Remove" }</Button>
                    <Button onclick={onretain}>{ "Retain" }</Button>
                    <Button onclick={onclear}>{ "Clear all" }</Button>
                    <p>
                        <b>{ "Current map: " }</b>
                    </p>
                    {
                        for map.current().iter().map(|(k, v)| {
                            html! {
                                <p><b>{ *k }</b> {": "} { v }</p>
                            }
                        })
                    }
                </div>
            </header>
        </div>
    }
}
