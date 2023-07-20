use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_search_param` demo
#[function_component]
pub fn UseSearchParam() -> Html {
    let param = use_search_param("foo".to_string());

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ " Current search param: " }</b>
                        { param.unwrap_or_default() }
                    </p>
                    <p>
                        {" click this link "} <a class="text-emerald-800 underline" href="?foo=123">{ "?foo=123" }</a>
                    </p>
                </div>
            </header>
        </div>
    }
}
