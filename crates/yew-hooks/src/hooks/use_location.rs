use gloo::utils::window;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::use_event_with_window;

/// State for brower's location.
pub struct LocationState {
    pub trigger: String,
    pub state: Option<JsValue>,
    pub length: u32,
    pub hash: String,
    pub host: String,
    pub hostname: String,
    pub href: String,
    pub origin: String,
    pub pathname: String,
    pub port: String,
    pub protocol: String,
    pub search: String,
}

/// A sensor hook that tracks brower's location value.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseLocation)]
/// fn location() -> Html {
///     let location = use_location();
///    
///     html! {
///         <>
///             <p>
///                 <b>{ "trigger: " }</b>
///                 { &location.trigger }
///             </p>
///             <p>
///                 <b>{ "state: " }</b>
///                 { format!("{:?}", &location.state) }
///             </p>
///             <p>
///                 <b>{ "length: " }</b>
///                 { &location.length }
///             </p>
///             <p>
///                 <b>{ "hash: " }</b>
///                 { &location.hash }
///             </p>
///             <p>
///                 <b>{ "host: " }</b>
///                 { &location.host }
///             </p>
///             <p>
///                 <b>{ "hostname: " }</b>
///                 { &location.hostname }
///             </p>
///             <p>
///                 <b>{ "href: " }</b>
///                 { &location.href }
///             </p>
///             <p>
///                 <b>{ "origin: " }</b>
///                 { &location.origin }
///             </p>
///             <p>
///                 <b>{ "pathname: " }</b>
///                 { &location.pathname }
///             </p>
///             <p>
///                 <b>{ "port: " }</b>
///                 { &location.port }
///             </p>
///             <p>
///                 <b>{ "protocol: " }</b>
///                 { &location.protocol }
///             </p>
///             <p>
///                 <b>{ "search: " }</b>
///                 { &location.search }
///             </p>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_location() -> UseStateHandle<LocationState> {
    let state = use_state(|| build_location("load".to_string()));

    {
        let state = state.clone();
        use_event_with_window("popstate", move |_: Event| {
            state.set(build_location("popstate".to_string()));
        });
    }

    {
        let state = state.clone();
        use_event_with_window("pushstate", move |_: Event| {
            state.set(build_location("pushstate".to_string()));
        });
    }

    {
        let state = state.clone();
        use_event_with_window("replacestate", move |_: Event| {
            state.set(build_location("replacestate".to_string()));
        });
    }

    state
}

fn build_location(trigger: String) -> LocationState {
    let location = window().location();
    let history = window().history().map_or((None, 0), |history| {
        (
            history.state().ok(),
            history.length().map_or(0, |length| length),
        )
    });

    LocationState {
        trigger,
        state: history.0,
        length: history.1,
        hash: location.hash().unwrap_or_default(),
        host: location.host().unwrap_or_default(),
        hostname: location.hostname().unwrap_or_default(),
        href: location.href().unwrap_or_default(),
        origin: location.origin().unwrap_or_default(),
        pathname: location.pathname().unwrap_or_default(),
        port: location.port().unwrap_or_default(),
        protocol: location.protocol().unwrap_or_default(),
        search: location.search().unwrap_or_default(),
    }
}
