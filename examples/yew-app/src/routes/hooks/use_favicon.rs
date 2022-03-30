use yew::prelude::*;

use yew_hooks::use_favicon;

/// `use_favicon` demo
#[function_component(UseFavicon)]
pub fn favicon() -> Html {
    use_favicon("https://crates.io/favicon.ico".to_string());

    html! {
        <>{ "View the favicon of this page" }</>
    }
}
