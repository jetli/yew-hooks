use std::ops::Deref;
use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use gloo::utils::{document, window};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::use_event_with_window;

/// State handle for the [`use_theme`] hook.
///
/// This handle allows toggling between light/dark themes, forcing system theme,
/// and syncing across tabs/windows.
pub struct UseThemeHandle {
    inner: UseStateHandle<String>,
    is_system: UseStateHandle<bool>,
    key: Rc<String>,
}

impl UseThemeHandle {
    /// Toggle between light and dark. This will persist the user's choice to localStorage.
    pub fn toggle(&self) {
        if *self.is_system {
            // If currently following system, toggling makes it explicit: choose opposite of current computed.
            let current = computed_theme();
            let next = if current == "dark" { "light" } else { "dark" }.to_string();
            LocalStorage::set(&*self.key, next.clone()).ok();
            self.inner.set(next);
            self.is_system.set(false);
        } else {
            let next = if self.inner.as_str() == "dark" {
                "light".to_string()
            } else {
                "dark".to_string()
            };
            LocalStorage::set(&*self.key, next.clone()).ok();
            self.inner.set(next);
            self.is_system.set(false);
        }
    }

    /// Set theme to dark and persist choice.
    pub fn set_dark(&self) {
        LocalStorage::set(&*self.key, "dark".to_string()).ok();
        self.inner.set("dark".to_string());
        self.is_system.set(false);
    }

    /// Set theme to light and persist choice.
    pub fn set_light(&self) {
        LocalStorage::set(&*self.key, "light".to_string()).ok();
        self.inner.set("light".to_string());
        self.is_system.set(false);
    }

    /// Follow system preference. This removes any persisted preference and uses the OS setting.
    pub fn set_system(&self) {
        LocalStorage::delete(&*self.key);
        let sys = computed_theme();
        self.inner.set(sys);
        self.is_system.set(true);
    }

    /// Returns true if the active theme is dark.
    pub fn is_dark(&self) -> bool {
        self.inner.as_str() == "dark"
    }
}

impl Deref for UseThemeHandle {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for UseThemeHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            is_system: self.is_system.clone(),
            key: self.key.clone(),
        }
    }
}

impl PartialEq for UseThemeHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner && *self.is_system == *other.is_system
    }
}

/// Helper calling `window.matchMedia(query)` and returning optionally whether it matches.
fn match_media_matches(query: &str) -> Option<bool> {
    let w = window();
    // call window.matchMedia via JS since web_sys types may not be available across versions
    let mm = js_sys::Reflect::get(&w, &JsValue::from_str("matchMedia")).ok()?;
    if mm.is_function() {
        let func: js_sys::Function = mm.dyn_into().ok()?;
        let mql = func.call1(&w.into(), &JsValue::from_str(query)).ok()?;
        let matches = js_sys::Reflect::get(&mql, &JsValue::from_str("matches")).ok()?;
        matches.as_bool()
    } else {
        None
    }
}

/// Compute the system theme using the `prefers-color-scheme` media query.
fn computed_theme() -> String {
    match match_media_matches("(prefers-color-scheme: dark)") {
        Some(true) => "dark".to_string(),
        _ => "light".to_string(),
    }
}

/// Apply or remove the `dark` class on the documentElement depending on theme.
/// This matches typical Tailwind usage when `darkMode: "class"`.
fn apply_theme_to_document(theme: &str) {
    if let Some(doc_el) = document().document_element() {
        // Read current `class` attribute and manipulate it manually so we don't rely on
        // `class_list` which may not be present across web_sys versions.
        let current = doc_el.get_attribute("class").unwrap_or_default();
        let mut parts: Vec<&str> = current
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect();
        let has_dark = parts.contains(&"dark");
        if theme == "dark" {
            if !has_dark {
                parts.push("dark");
            }
        } else {
            parts.retain(|&p| p != "dark");
        }
        let new = parts.join(" ");
        let _ = if new.is_empty() {
            doc_el.remove_attribute("class")
        } else {
            doc_el.set_attribute("class", &new)
        };
        // Also set data-theme for convenience
        let _ = doc_el.set_attribute("data-theme", theme);
    }
}

/// A hook to manage light/dark theme with persistence and system preference support.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseTheme)]
/// fn theme() -> Html {
///     let theme = use_theme("example_theme".to_string());
///
///     let onclick = {
///         let theme = theme.clone();
///         Callback::from(move |_| theme.toggle())
///     };
///
///     html! {
///         <>
///             <button {onclick}>
///                 { if theme.is_dark() { "Switch to light" } else { "Switch to dark" } }
///             </button>
///             <p>{ format!("Active theme: {}", *theme) }</p>
///         </>
///     }
/// }
/// ```
///
/// - `key`: the localStorage key to persist the user's preference. If not present, the hook follows system preference.
///
/// Returns a `UseThemeHandle` that can be used to read the current theme and switch it.
#[hook]
pub fn use_theme(key: String) -> UseThemeHandle {
    // Determine initial state:
    // If localStorage has a value, use it and mark not system.
    // Otherwise, follow system and mark system = true.
    let stored = LocalStorage::get::<String>(&key).ok();
    let (initial_theme, initial_is_system) = match stored {
        Some(s) if s == "dark" || s == "light" => (s, false),
        _ => (computed_theme(), true),
    };

    let inner = use_state(|| initial_theme.clone());
    let is_system = use_state(|| initial_is_system);
    let key_rc = use_memo((), |_| key);

    // Apply theme to document when theme changes
    {
        let inner = inner.clone();
        use_effect_with(inner.clone(), move |theme| {
            apply_theme_to_document(theme.as_str());
            || ()
        });
    }

    // Listen to storage events to sync across tabs/windows
    {
        let inner = inner.clone();
        let is_system = is_system.clone();
        let key = key_rc.clone();
        use_event_with_window("storage", move |e: web_sys::Event| {
            // The storage event exposes `key` property on the JS event object.
            let js_e = JsValue::from(e);
            let key_from_event = js_sys::Reflect::get(&js_e, &JsValue::from_str("key"))
                .ok()
                .and_then(|v| v.as_string());
            if let Some(k) = key_from_event {
                if k == *key {
                    // Read latest value from localStorage. If missing, switch to system.
                    if let Ok(v) = LocalStorage::get::<String>(&*key) {
                        if v == "dark" || v == "light" {
                            inner.set(v);
                            is_system.set(false);
                        } else {
                            // unknown value => treat as system
                            inner.set(computed_theme());
                            is_system.set(true);
                        }
                    } else {
                        inner.set(computed_theme());
                        is_system.set(true);
                    }
                }
            }
        });
    }

    // Listen to system preference change and update only when following system.
    // Use JS interop to attach an event listener on the MediaQueryList object.
    {
        let inner = inner.clone();
        let is_system = is_system.clone();
        use_effect_with((), move |_| {
            // Obtain MediaQueryList via JS
            let w = window();
            let mm = js_sys::Reflect::get(&w, &JsValue::from_str("matchMedia")).ok();
            let mql = mm
                .and_then(|mm| mm.dyn_into::<js_sys::Function>().ok())
                .and_then(|f| {
                    f.call1(
                        &w.into(),
                        &JsValue::from_str("(prefers-color-scheme: dark)"),
                    )
                    .ok()
                });

            if let Some(mql_val) = mql {
                // initial sync if following system
                if *is_system {
                    let matches = js_sys::Reflect::get(&mql_val, &JsValue::from_str("matches"))
                        .ok()
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    inner.set(if matches {
                        "dark".to_string()
                    } else {
                        "light".to_string()
                    });
                }

                // add event listener for changes, only update when following system
                // Cast to EventTarget to use add_event_listener_with_callback
                if let Ok(target) = mql_val.clone().dyn_into::<web_sys::EventTarget>() {
                    let inner_cl = inner.clone();
                    let is_system_cl = is_system.clone();
                    // keep mql_val clone inside closure so we can read `.matches`
                    let mql_for_closure = mql_val.clone();
                    let closure = Closure::wrap(Box::new(move |_ev: JsValue| {
                        if *is_system_cl {
                            let matches = js_sys::Reflect::get(
                                &mql_for_closure,
                                &JsValue::from_str("matches"),
                            )
                            .ok()
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                            inner_cl.set(if matches {
                                "dark".to_string()
                            } else {
                                "light".to_string()
                            });
                        }
                    }) as Box<dyn FnMut(_)>);

                    let _ = target.add_event_listener_with_callback(
                        "change",
                        closure.as_ref().unchecked_ref(),
                    );
                    // Leak the closure intentionally so it lives for the page lifetime (consistent with other hooks).
                    closure.forget();
                }
            }

            || ()
        });
    }

    UseThemeHandle {
        inner,
        is_system,
        key: key_rc,
    }
}
