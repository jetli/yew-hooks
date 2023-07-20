use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_favicon` demo
#[function_component]
pub fn UseFavicon() -> Html {
    use_favicon("https://crates.io/favicon.ico".to_string());

    html! {
        <>{ "View the favicon of this page" }</>
    }
}
