use std::cell::{Ref, RefCell};
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use yew::prelude::*;

use super::use_update;

/// State handle for the [`use_set`] hook.
pub struct UseSetHandle<T> {
    inner: Rc<RefCell<HashSet<T>>>,
    update: Rc<dyn Fn()>,
}

impl<T> UseSetHandle<T> {
    /// Get immutable ref to the set.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed
    pub fn current(&'_ self) -> Ref<'_, HashSet<T>> {
        self.inner.borrow()
    }

    /// Set the hash set.
    pub fn set(&self, set: HashSet<T>) {
        *self.inner.borrow_mut() = set;
        (self.update)();
    }

    /// Adds a value to the set.
    pub fn insert(&self, value: T) -> bool
    where
        T: Eq + Hash,
    {
        let present = self.inner.borrow_mut().insert(value);
        (self.update)();
        present
    }

    /// Adds a value to the set, replacing the existing value,
    /// if any, that is equal to the given one. Returns the replaced value.
    pub fn replace(&self, value: T) -> Option<T>
    where
        T: Eq + Hash,
    {
        let v = self.inner.borrow_mut().replace(value);
        (self.update)();
        v
    }

    /// Removes a value from the set. Returns whether the value was present in the set.
    pub fn remove(&self, value: &T) -> bool
    where
        T: Eq + Hash,
    {
        let present = self.inner.borrow_mut().remove(value);
        (self.update)();
        present
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&self, f: F)
    where
        T: Eq + Hash,
        F: FnMut(&T) -> bool,
    {
        self.inner.borrow_mut().retain(f);
        (self.update)();
    }

    /// Clears the set, removing all values.
    pub fn clear(&self) {
        self.inner.borrow_mut().clear();
        (self.update)();
    }
}

impl<T> Clone for UseSetHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            update: self.update.clone(),
        }
    }
}

impl<T> PartialEq for UseSetHandle<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A hook that tracks a hash set and provides methods to modify it.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashSet;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseSet)]
/// fn set() -> Html {
///     let set = use_set(HashSet::from(["Mercury", "Venus", "Earth", "Mars"]));
///
///     let onset = {
///         let set = set.clone();
///         Callback::from(move |_| set.set(HashSet::from(["Moon", "Earth"])))
///     };
///     let oninsert = {
///         let set = set.clone();
///         Callback::from(move |_| {
///             let _ = set.insert("Jupiter");
///         })
///     };
///     let onreplace = {
///         let set = set.clone();
///         Callback::from(move |_| {
///             let _ = set.replace("Earth");
///         })
///     };
///     let onremove = {
///         let set = set.clone();
///         Callback::from(move |_| {
///             let _ = set.remove(&"Moon");
///         })
///     };
///     let onretain = {
///         let set = set.clone();
///         Callback::from(move |_| set.retain(|v| v.contains('a')))
///     };
///     let onclear = {
///         let set = set.clone();
///         Callback::from(move |_| set.clear())
///     };
///
///     html! {
///         <div>
///             <button onclick={onset}>{ "Set" }</button>
///             <button onclick={oninsert}>{ "Insert" }</button>
///             <button onclick={onreplace}>{ "Replace" }</button>
///             <button onclick={onremove}>{ "Remove" }</button>
///             <button onclick={onretain}>{ "Retain" }</button>
///             <button onclick={onclear}>{ "Clear all" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///             </p>
///             {
///                 for set.current().iter().map(|v| {
///                     html! {
///                         <p><b>{ *v }</b></p>
///                     }
///                 })
///             }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_set<T>(initial_value: HashSet<T>) -> UseSetHandle<T>
where
    T: 'static,
{
    let inner = use_mut_ref(|| initial_value);
    let update = use_update();

    UseSetHandle { inner, update }
}
