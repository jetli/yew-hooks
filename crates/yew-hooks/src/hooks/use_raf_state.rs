use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

use gloo::render::{request_animation_frame, AnimationFrame};

use yew::prelude::*;

use super::use_unmount;

/// State handle for the [`use_raf_state`] hook.
pub struct UseRafStateHandle<T> {
    inner: UseStateHandle<T>,
    raf: Rc<RefCell<Option<AnimationFrame>>>,
}

impl<T> UseRafStateHandle<T>
where
    T: 'static,
{
    /// Replaces the value.
    pub fn set(&self, value: T) {
        let inner = self.inner.clone();
        *self.raf.borrow_mut() = Some(request_animation_frame(move |_| {
            inner.set(value);
        }));
    }
}

impl<T> Deref for UseRafStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl<T> Clone for UseRafStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            raf: self.raf.clone(),
        }
    }
}

impl<T> PartialEq for UseRafStateHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A state hook that only updates state in the callback of `requestAnimationFrame`.
///
/// # Example
///
/// ```rust
/// # use web_sys::Window;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::{use_event_with_window, use_raf_state};
///
/// #[function_component(UseRafState)]
/// fn raf_state() -> Html {
///     let state = use_raf_state(|| (0f64, 0f64));
///
///     {
///         let state = state.clone();
///         use_event_with_window("resize", move |e: Event| {
///             let window: Window = e.target_unchecked_into();
///             state.set((
///                 window.inner_width().unwrap().as_f64().unwrap(),
///                 window.inner_height().unwrap().as_f64().unwrap(),
///             ));
///         });
///     }
///     
///     html! {
///         <>
///             <b>{ " Width: " }</b>
///             { state.0 }
///             <b>{ " Height: " }</b>
///             { state.1 }
///         </>
///     }
/// }
/// ```
pub fn use_raf_state<T, F>(init_fn: F) -> UseRafStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let inner = use_state(init_fn);
    let raf = use_mut_ref(|| None);

    {
        let raf = raf.clone();
        use_unmount(move || {
            *raf.borrow_mut() = None;
        });
    }

    UseRafStateHandle { inner, raf }
}
