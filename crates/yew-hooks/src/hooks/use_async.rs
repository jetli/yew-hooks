use std::future::Future;
use std::ops::Deref;

use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;

use super::{use_mut_latest, UseMutLatestHandle};

/// State for an async future.
#[derive(PartialEq)]
pub struct UseAsyncState<T, E> {
    pub loading: bool,
    pub data: Option<T>,
    pub error: Option<E>,
}

/// State handle for the [`use_async`] hook.
pub struct UseAsyncHandle<F, T, E> {
    inner: UseStateHandle<UseAsyncState<T, E>>,
    future_ref: UseMutLatestHandle<Option<F>>,
}

impl<F, T, E> UseAsyncHandle<F, T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Clone + 'static,
    E: Clone + 'static,
{
    /// Start to resolve the async future to a final value.
    pub fn run(self) {
        spawn_local(async move {
            let future_ref = self.future_ref.current();
            let future = (*future_ref.borrow_mut()).take();

            if let Some(future) = future {
                // Only set loading to true and leave previous data/error alone.
                self.inner.set(UseAsyncState {
                    loading: true,
                    data: (*self.inner).data.clone(),
                    error: (*self.inner).error.clone(),
                });
                match future.await {
                    // Success with some data and clear previous error.
                    Ok(data) => self.inner.set(UseAsyncState {
                        loading: false,
                        data: Some(data),
                        error: None,
                    }),
                    // Failed with some error and leave previous data alone.
                    Err(error) => self.inner.set(UseAsyncState {
                        loading: false,
                        data: (*self.inner).data.clone(),
                        error: Some(error),
                    }),
                }
            }
        });
    }
}

impl<F, T, E> Deref for UseAsyncHandle<F, T, E> {
    type Target = UseAsyncState<T, E>;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl<F, T, E> Clone for UseAsyncHandle<F, T, E> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            future_ref: self.future_ref.clone(),
        }
    }
}

impl<F, T, E> PartialEq for UseAsyncHandle<F, T, E>
where
    T: PartialEq,
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// This hook returns state and a `run` callback for an async future.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_async;
///
/// #[function_component(Async)]
/// fn async_test() -> Html {
///     let state = use_async(async move {
///         fetch("/api/user/123".to_string()).await
///     });
///
///     let onclick = {
///         let state = state.clone();
///         Callback::from(move |_| {
///             let state = state.clone();
///             state.run();
///         })
///     };
///     
///     html! {
///         <div>
///             <button {onclick} disabled={state.loading}>{ "Start loading" }</button>
///             {
///                 if state.loading {
///                     html! { "Loading" }
///                 } else {
///                     html! {}
///                 }
///             }
///             {
///                 if let Some(data) = &state.data {
///                     html! { data }
///                 } else {
///                     html! {}
///                 }
///             }
///             {
///                 if let Some(error) = &state.error {
///                     html! { error }
///                 } else {
///                     html! {}
///                 }
///             }
///         </div>
///     }
/// }
///
/// async fn fetch(url: String) -> Result<String, String> {
///     // You can use reqwest to fetch your http api
///     Ok(String::from("Jet Li"))
/// }
/// ```
pub fn use_async<F, T, E>(future: F) -> UseAsyncHandle<F, T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: 'static,
    E: 'static,
{
    let inner = use_state(|| UseAsyncState {
        loading: false,
        data: None,
        error: None,
    });
    let future_ref = use_mut_latest(Some(future));

    UseAsyncHandle { inner, future_ref }
}
