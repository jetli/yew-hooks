use yew::prelude::*;

use yew_hooks::use_search_param;

/// `use_search_param` demo
#[function_component(UseSearchParam)]
pub fn search_param() -> Html {
    let param = use_search_param("foo".to_string());

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ " Current search param: " }</b>
                        { param.unwrap_or_default() }
                    </p>
                    <p>
                        {" click this link "} <a href="?foo=123">{ "?foo=123" }</a>
                    </p>
                </div>
            </header>
        </div>
    }
}
