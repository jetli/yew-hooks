use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use yew::prelude::*;

use super::use_update;

/// State handle for the [`use_map`] hook.
pub struct UseMapHandle<K, V> {
    inner: Rc<RefCell<HashMap<K, V>>>,
    update: Rc<dyn Fn()>,
}

impl<K, V> UseMapHandle<K, V> {
    /// Get immutable ref to the map.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed
    pub fn current(&self) -> Ref<HashMap<K, V>> {
        self.inner.borrow()
    }

    /// Set the hash map.
    pub fn set(&self, map: HashMap<K, V>) {
        *self.inner.borrow_mut() = map;
        (self.update)();
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&self, k: K, v: V) -> Option<V>
    where
        K: Eq + Hash,
    {
        let v = self.inner.borrow_mut().insert(k, v);
        (self.update)();
        v
    }

    /// Update key-value pair.
    pub fn update(&self, k: &K, v: V)
    where
        K: Eq + Hash,
    {
        if let Some(value) = self.inner.borrow_mut().get_mut(k) {
            *value = v;
        }
        (self.update)();
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove(&self, k: &K) -> Option<V>
    where
        K: Eq + Hash,
    {
        let v = self.inner.borrow_mut().remove(k);
        (self.update)();
        v
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&self, f: F)
    where
        K: Eq + Hash,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.borrow_mut().retain(f);
        (self.update)();
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&self) {
        self.inner.borrow_mut().clear();
        (self.update)();
    }
}

impl<K, V> Clone for UseMapHandle<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            update: self.update.clone(),
        }
    }
}

impl<K, V> PartialEq for UseMapHandle<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A hook that tracks a hash map and provides methods to modify it.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashMap;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseMap)]
/// fn map() -> Html {
///     let map = use_map(HashMap::from([
///         ("Mercury", 0.4),
///         ("Venus", 0.7),
///         ("Earth", 1.0),
///         ("Mars", 1.5),
///     ]));
///
///     let onset = {
///         let map = map.clone();
///         Callback::from(move |_| map.set(HashMap::from([("Moon", 0.8), ("Earth", 1.0)])))
///     };
///     let oninsert = {
///         let map = map.clone();
///         Callback::from(move |_| {
///             let _ = map.insert("Jupiter", 2.1);
///         })
///     };
///     let onupdate = {
///         let map = map.clone();
///         Callback::from(move |_| map.update(&"Earth", 1.1))
///     };
///     let onremove = {
///         let map = map.clone();
///         Callback::from(move |_| {
///             let _ = map.remove(&"Moon");
///         })
///     };
///     let onretain = {
///         let map = map.clone();
///         Callback::from(move |_| map.retain(|_k, v| v > &mut 1.0))
///     };
///     let onclear = {
///         let map = map.clone();
///         Callback::from(move |_| map.clear())
///     };
///
///     html! {
///         <div>
///             <button onclick={onset}>{ "Set" }</button>
///             <button onclick={oninsert}>{ "Insert" }</button>
///             <button onclick={onupdate}>{ "Update" }</button>
///             <button onclick={onremove}>{ "Remove" }</button>
///             <button onclick={onretain}>{ "Retain" }</button>
///             <button onclick={onclear}>{ "Clear all" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///             </p>
///             {
///                 for map.current().iter().map(|(k, v)| {
///                     html! {
///                         <p><b>{ k }</b> {": "} { v }</p>
///                     }
///                 })
///             }
///         </div>
///     }
/// }
/// ```
pub fn use_map<K, V>(initial_value: HashMap<K, V>) -> UseMapHandle<K, V>
where
    K: 'static,
    V: 'static,
{
    let inner = use_mut_ref(|| initial_value);
    let update = use_update();

    UseMapHandle { inner, update }
}
