use std::ops::Deref;
use std::rc::Rc;

use gloo::utils::document;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// State handle for the [`use_cookie`] hook.
pub struct UseCookieHandle<T> {
    inner: UseStateHandle<Option<T>>,
    key: Rc<String>,
}

impl<T> UseCookieHandle<T> {
    /// Set a `value` for the specified key.
    pub fn set(&self, value: T)
    where
        T: Serialize + Clone,
    {
        if let Ok(cookie_str) = serde_json::to_string(&value) {
            // URL encode the value
            let encoded_value = urlencoding::encode(&cookie_str);
            let cookie = format!("{}={}", self.key, encoded_value);
            set_cookie(&cookie);
            self.inner.set(Some(value));
        }
    }

    /// Set a `value` for the specified key with additional cookie attributes.
    pub fn set_with_attributes(&self, value: T, attributes: CookieAttributes)
    where
        T: Serialize + Clone,
    {
        if let Ok(cookie_str) = serde_json::to_string(&value) {
            // URL encode the value
            let encoded_value = urlencoding::encode(&cookie_str);
            let mut cookie = format!("{}={}", self.key, encoded_value);

            if let Some(max_age) = attributes.max_age {
                cookie.push_str(&format!("; max-age={}", max_age));
            }
            if let Some(path) = &attributes.path {
                cookie.push_str(&format!("; path={}", path));
            }
            if let Some(domain) = &attributes.domain {
                cookie.push_str(&format!("; domain={}", domain));
            }
            if attributes.secure {
                cookie.push_str("; secure");
            }
            // Note: HttpOnly cookies cannot be set or accessed via JavaScript
            // They can only be set server-side via HTTP headers
            if let Some(same_site) = &attributes.same_site {
                cookie.push_str(&format!("; SameSite={}", same_site));
            }

            set_cookie(&cookie);
            self.inner.set(Some(value));
        }
    }

    /// Delete a key and its stored value.
    pub fn delete(&self) {
        // Set expiration date in the past to delete the cookie
        let cookie = format!("{}=; expires=Thu, 01 Jan 1970 00:00:00 GMT", self.key);
        set_cookie(&cookie);
        self.inner.set(None);
    }
}

impl<T> Deref for UseCookieHandle<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseCookieHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            key: self.key.clone(),
        }
    }
}

impl<T> PartialEq for UseCookieHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// Attributes for setting cookies
#[derive(Debug, Clone, Default)]
pub struct CookieAttributes {
    /// Maximum age of the cookie in seconds
    pub max_age: Option<i64>,
    /// Path for which the cookie is valid
    pub path: Option<String>,
    /// Domain for which the cookie is valid
    pub domain: Option<String>,
    /// If true, the cookie is only sent over HTTPS
    pub secure: bool,
    /// Note: HttpOnly cannot be set via JavaScript - it's only for server-side cookies
    /// This field is kept for API consistency but has no effect
    pub http_only: bool,
    /// SameSite attribute for the cookie
    pub same_site: Option<SameSite>,
}

/// SameSite attribute values
#[derive(Debug, Clone)]
pub enum SameSite {
    /// Strict same-site policy
    Strict,
    /// Lax same-site policy
    Lax,
    /// No same-site restriction
    None,
}

impl std::fmt::Display for SameSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SameSite::Strict => write!(f, "Strict"),
            SameSite::Lax => write!(f, "Lax"),
            SameSite::None => write!(f, "None"),
        }
    }
}

/// Set a cookie using JavaScript's document.cookie
fn set_cookie(cookie: &str) {
    let doc = document();
    let _ = js_sys::Reflect::set(
        &doc,
        &JsValue::from_str("cookie"),
        &JsValue::from_str(cookie),
    );
}

/// Get the cookie string from document.cookie
fn get_cookie_string() -> Option<String> {
    let doc = document();
    js_sys::Reflect::get(&doc, &JsValue::from_str("cookie"))
        .ok()
        .and_then(|v| v.as_string())
}

/// Parse a cookie string to get the value for a specific key
fn get_cookie_value(key: &str) -> Option<String> {
    get_cookie_string().and_then(|cookie_str| {
        cookie_str
            .split(';')
            .map(|cookie| cookie.trim())
            .find(|cookie| cookie.starts_with(&format!("{}=", key)))
            .and_then(|cookie| cookie.split('=').nth(1))
            .map(|value| value.to_string())
    })
}

/// A side-effect hook that manages a single cookie.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Cookie)]
/// fn cookie() -> Html {
///     let cookie = use_cookie::<String>("foo".to_string());
///
///     let onclick = {
///         let cookie = cookie.clone();
///         Callback::from(move |_| cookie.set("bar".to_string()))
///     };
///     let ondelete = {
///         let cookie = cookie.clone();
///         Callback::from(move |_| cookie.delete())
///     };
///
///     html! {
///         <div>
///             <button onclick={onclick}>{ "Set to bar" }</button>
///             <button onclick={ondelete}>{ "Delete" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 {
///                     if let Some(value) = &*cookie {
///                         html! { value }
///                     } else {
///                         html! {}
///                     }
///                 }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_cookie<T>(key: String) -> UseCookieHandle<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    let inner: UseStateHandle<Option<T>> = use_state(|| {
        get_cookie_value(&key).and_then(|value| {
            // URL decode the value
            let decoded_value = urlencoding::decode(&value).ok()?;
            serde_json::from_str(&decoded_value).ok()
        })
    });
    let key = use_memo((), |_| key);

    UseCookieHandle { inner, key }
}
