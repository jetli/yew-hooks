use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::rc::Rc;

use yew::prelude::*;

use super::use_update;

/// State handle for the [`use_list`] hook.
pub struct UseListHandle<T> {
    inner: Rc<RefCell<Vec<T>>>,
    update: Rc<dyn Fn()>,
}

impl<T> UseListHandle<T> {
    /// Get immutable ref to the list.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed
    pub fn current(&'_ self) -> Ref<'_, Vec<T>> {
        self.inner.borrow()
    }

    /// Set the list by `elements`
    pub fn set(&self, elements: Vec<T>) {
        *self.inner.borrow_mut() = elements;
        (self.update)();
    }

    /// Insert an element at the specified index
    ///
    /// # Panics
    ///
    /// Panics if `index` > `len`.
    pub fn insert(&self, index: usize, element: T) {
        self.inner.borrow_mut().insert(index, element);
        (self.update)();
    }

    /// Update the element at the specified index
    pub fn update(&self, index: usize, element: T) {
        if let Some(elem) = self.inner.borrow_mut().get_mut(index) {
            *elem = element;
        }
        (self.update)();
    }

    /// Removes and returns the element at position index within the list,
    /// shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    pub fn remove(&self, index: usize) -> T {
        let elem = self.inner.borrow_mut().remove(index);
        (self.update)();
        elem
    }

    /// Appends an element to the back of a collection.
    pub fn push(&self, value: T) {
        self.inner.borrow_mut().push(value);
        (self.update)();
    }

    /// Removes the last element from a vector and returns it, or None if it is empty.
    pub fn pop(&self) -> Option<T> {
        let value = self.inner.borrow_mut().pop();
        (self.update)();
        value
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner.borrow_mut().retain(f);
        (self.update)();
    }

    /// Reverses the order of elements in the slice, in place.
    pub fn reverse(&self) {
        self.inner.borrow_mut().reverse();
        (self.update)();
    }

    /// Moves all the elements of other into Self, leaving other empty.
    pub fn append(&self, other: &mut Vec<T>) {
        self.inner.borrow_mut().append(other);
        (self.update)();
    }

    /// Sorts the list.
    pub fn sort(&self)
    where
        T: Ord,
    {
        self.inner.borrow_mut().sort();
        (self.update)();
    }

    /// Sorts the list with a comparator function.
    pub fn sort_by<F>(&self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.inner.borrow_mut().sort_by(compare);
        (self.update)();
    }

    /// Swaps two elements in the list.
    pub fn swap(&self, a: usize, b: usize) {
        self.inner.borrow_mut().swap(a, b);
        (self.update)();
    }

    /// Clears the list, removing all values.
    pub fn clear(&self) {
        self.inner.borrow_mut().clear();
        (self.update)();
    }
}

impl<T> Clone for UseListHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            update: self.update.clone(),
        }
    }
}

impl<T> PartialEq for UseListHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A hook that tracks a list and provides methods to modify it.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseList)]
/// fn list() -> Html {
///     let list = use_list(vec![1, 2, 3, 4, 5]);
///     
///     let onset = {
///         let list = list.clone();
///         Callback::from(move |_| list.set(vec![6, 7, 8, 9, 10]))
///     };
///     let oninsert = {
///         let list = list.clone();
///         Callback::from(move |_| list.insert(0, 9))
///     };
///     let onupdate = {
///         let list = list.clone();
///         Callback::from(move |_| list.update(0, 4))
///     };
///     let onremove = {
///         let list = list.clone();
///         Callback::from(move |_| { let _ = list.remove(0); })
///     };
///     let onretain = {
///         let list = list.clone();
///         Callback::from(move |_| list.retain(|x| x % 2 == 0))
///     };
///     let onclear = {
///         let list = list.clone();
///         Callback::from(move |_| list.clear())
///     };
///
///     html! {
///         <div>
///             <button onclick={onset}>{ "Set to [6, 7, 8, 9, 10]" }</button>
///             <button onclick={oninsert}>{ "Insert 9 at position 0" }</button>
///             <button onclick={onupdate}>{ "Update to 4 at position 0" }</button>
///             <button onclick={onremove}>{ "Remove position 0" }</button>
///             <button onclick={onretain}>{ "Retain even numbers" }</button>
///             <button onclick={onclear}>{ "Clear all" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 {
///                     for list.current().iter().map(|element| {
///                         html! { element }
///                     })
///                 }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_list<T>(initial_value: Vec<T>) -> UseListHandle<T>
where
    T: 'static,
{
    let inner = use_mut_ref(|| initial_value);
    let update = use_update();

    UseListHandle { inner, update }
}
