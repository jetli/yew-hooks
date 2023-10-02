use std::ops::Deref;
use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::StorageEvent;
use yew::prelude::*;

use super::use_event_with_window;

/// State handle for the [`use_local_storage`] hook.
pub struct UseLocalStorageHandle<T> {
    inner: UseStateHandle<Option<T>>,
    key: Rc<String>,
}

impl<T> UseLocalStorageHandle<T> {
    /// Set a `value` for the specified key.
    pub fn set(&self, value: T)
    where
        T: Serialize + Clone,
    {
        if LocalStorage::set(&*self.key, value.clone()).is_ok() {
            self.inner.set(Some(value));
        }
    }

    /// Delete a key and it's stored value.
    pub fn delete(&self) {
        LocalStorage::delete(&*self.key);
        self.inner.set(None);
    }
}

impl<T> Deref for UseLocalStorageHandle<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseLocalStorageHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            key: self.key.clone(),
        }
    }
}

impl<T> PartialEq for UseLocalStorageHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A side-effect hook that manages a single localStorage key.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(LocalStorage)]
/// fn local_storage() -> Html {
///     let storage = use_local_storage::<String>("foo".to_string());
///     
///     let onclick = {
///         let storage = storage.clone();
///         Callback::from(move |_| storage.set("bar".to_string()))
///     };
///     let ondelete = {
///         let storage = storage.clone();
///         Callback::from(move |_| storage.delete())
///     };
///
///     html! {
///         <div>
///             <button onclick={onclick}>{ "Set to bar" }</button>
///             <button onclick={ondelete}>{ "Delete" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 {
///                     if let Some(value) = &*storage {
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
pub fn use_local_storage<T>(key: String) -> UseLocalStorageHandle<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    let inner: UseStateHandle<Option<T>> =
        use_state(|| LocalStorage::get(&key).unwrap_or_default());
    let key = use_memo((), |_| key);

    {
        let key = key.clone();
        let inner = inner.clone();
        use_event_with_window("storage", move |e: StorageEvent| {
            if let Some(k) = e.key() {
                if k == *key {
                    inner.set(LocalStorage::get(&*key).unwrap_or_default());
                }
            }
        });
    }

    UseLocalStorageHandle { inner, key }
}
