use gloo::utils::window;
use web_sys::UrlSearchParams;

use yew::prelude::*;

use super::use_event_with_window;

/// A sensor hook that tracks brower's location search param value.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_search_param;
///
/// #[function_component(UseSearchParam)]
/// fn search_param() -> Html {
///     let param = use_search_param("foo".to_string());
///    
///     html! {
///         <>
///             <p>
///                 <b>{ " Current search param: " }</b>
///                 { param.unwrap_or_default() }
///             </p>
///         </>
///     }
/// }
/// ```
pub fn use_search_param(param: String) -> Option<String> {
    let state = use_state(|| get_param(param.clone()));

    {
        let state = state.clone();
        let param = param.clone();
        use_event_with_window("popstate", move |_: Event| {
            let param = param.clone();
            state.set(get_param(param));
        });
    }

    {
        let state = state.clone();
        let param = param.clone();
        use_event_with_window("pushstate", move |_: Event| {
            let param = param.clone();
            state.set(get_param(param));
        });
    }

    {
        let state = state.clone();
        use_event_with_window("replacestate", move |_: Event| {
            let param = param.clone();
            state.set(get_param(param));
        });
    }

    (*state).clone()
}

fn get_param(param: String) -> Option<String> {
    let search = window().location().search().unwrap_or_default();
    UrlSearchParams::new_with_str(search.as_str()).map_or(None, |params| params.get(param.as_str()))
}
