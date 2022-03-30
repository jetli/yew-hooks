use gloo::utils::document;
use web_sys::{HtmlLinkElement, Node};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

/// A side-effect hook that sets favicon of the page.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_favicon;
///
/// #[function_component(Favicon)]
/// fn favicon() -> Html {
///     use_favicon("https://crates.io/favicon.ico".to_string());
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
pub fn use_favicon(href: String) {
    use_effect_with_deps(
        move |href| {
            let link = {
                if let Ok(Some(link)) = document().query_selector("link[rel*='icon']") {
                    link
                } else {
                    document().create_element("link").unwrap_throw()
                }
            }
            .dyn_into::<HtmlLinkElement>()
            .unwrap_throw();

            link.set_type("image/x-icon");
            link.set_rel("shortcut icon");
            link.set_href(href);

            let head = document()
                .get_elements_by_tag_name("head")
                .item(0)
                .unwrap_throw();
            let _ = head.append_child(&link.dyn_into::<Node>().unwrap_throw());

            || ()
        },
        href,
    )
}
