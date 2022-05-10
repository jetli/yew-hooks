use std::ops::Deref;

use gloo::utils::window;
use yew::prelude::*;

use super::use_event_with_window;

/// State handle for the [`use_hash`] hook.
pub struct UseHashHandle {
    inner: UseStateHandle<String>,
}

impl UseHashHandle {
    pub fn set(&self, hash: String) {
        if *self.inner != hash {
            let _ = window().location().set_hash(hash.as_str());
        }
    }
}

impl Deref for UseHashHandle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseHashHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl PartialEq for UseHashHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A sensor hook that tracks brower's location hash value.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseHash)]
/// fn hash() -> Html {
///     let hash = use_hash();
///
///     let onclick = {
///         let hash = hash.clone();
///         Callback::from(move |_| {
///             hash.set("#/path/to/page?userId=123".to_string())
///         })
///     };
///    
///     html! {
///         <>
///             <button {onclick}>{ "Set hash to #/path/to/page?userId=123" }</button>
///             <p>
///                 <b>{ " Current hash: " }</b>
///                 { &*hash }
///             </p>
///         </>
///     }
/// }
/// ```
pub fn use_hash() -> UseHashHandle {
    let inner = use_state(|| window().location().hash().unwrap_or_default());

    {
        let inner = inner.clone();
        use_event_with_window("hashchange", move |_: Event| {
            inner.set(window().location().hash().unwrap_or_default());
        });
    }

    UseHashHandle { inner }
}
