use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_hash` demo
#[function_component(UseHash)]
pub fn hash() -> Html {
    let hash = use_hash();

    let onclick = {
        let hash = hash.clone();
        Callback::from(move |_| {
            hash.set("#/path/to/page?userId=123".to_string());
        })
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ " Current hash: " }</b>
                        { &*hash }
                    </p>
                    <p>
                        <button {onclick}>{ "Set hash to #/path/to/page?userId=123" }</button><br/>
                        {" or click a link "} <a href="#/path/to/page?userId=456">{ "#/path/to/page?userId=456" }</a>
                    </p>
                </div>
            </header>
        </div>
    }
}
