use std::ops::Deref;
use std::rc::Rc;

use gloo::storage::{SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

/// State handle for the [`use_session_storage`] hook.
pub struct UseSessionStorageHandle<T> {
    inner: UseStateHandle<Option<T>>,
    key: Rc<String>,
}

impl<T> UseSessionStorageHandle<T> {
    /// Set a `value` for the specified key.
    pub fn set(&self, value: T)
    where
        T: Serialize + Clone,
    {
        if SessionStorage::set(&*self.key, value.clone()).is_ok() {
            self.inner.set(Some(value));
        }
    }

    /// Delete a key and it's stored value.
    pub fn delete(&self) {
        SessionStorage::delete(&*self.key);
        self.inner.set(None);
    }
}

impl<T> Deref for UseSessionStorageHandle<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseSessionStorageHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            key: self.key.clone(),
        }
    }
}

impl<T> PartialEq for UseSessionStorageHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A side-effect hook that manages a single sessionStorage key.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(SessionStorage)]
/// fn session_storage() -> Html {
///     let storage = use_session_storage::<String>("foo".to_string());
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
pub fn use_session_storage<T>(key: String) -> UseSessionStorageHandle<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    let inner: UseStateHandle<Option<T>> =
        use_state(|| SessionStorage::get(&key).unwrap_or_default());
    let key = use_memo((), |_| key);

    UseSessionStorageHandle { inner, key }
}
