use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_hash` demo
#[function_component]
pub fn UseHash() -> Html {
    let hash = use_hash();

    let onclick = {
        let hash = hash.clone();
        Callback::from(move |_| {
            hash.set("#/path/to/page?userId=123".to_string());
        })
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ " Current hash: " }</b>
                        { &*hash }
                    </p>
                    <p>
                        <Button {onclick}>{ "Set hash to #/path/to/page?userId=123" }</Button><br/>
                        {" or click a link "} <a href="#/path/to/page?userId=456">{ "#/path/to/page?userId=456" }</a>
                    </p>
                </div>
            </header>
        </div>
    }
}
