use std::ops::Deref;
use std::{future::Future, rc::Rc};

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::{use_mount, use_mut_latest};

/// Options for [`use_async_with_options`].
#[derive(Default)]
pub struct UseAsyncOptions {
    pub auto: bool,
}

impl UseAsyncOptions {
    /// Automatically run when mount
    pub fn enable_auto() -> Self {
        Self { auto: true }
    }
}

/// State for an async future.
#[derive(PartialEq)]
pub struct UseAsyncState<T, E> {
    pub loading: bool,
    pub data: Option<T>,
    pub error: Option<E>,
}

/// State handle for the [`use_async`] hook.
pub struct UseAsyncHandle<T, E> {
    inner: UseStateHandle<UseAsyncState<T, E>>,
    run: Rc<dyn Fn()>,
}

impl<T, E> UseAsyncHandle<T, E> {
    /// Start to resolve the async future to a final value.
    pub fn run(&self) {
        (self.run)()
    }

    /// Update `data` directly.
    pub fn update(&self, data: T) {
        self.inner.set(UseAsyncState {
            loading: false,
            data: Some(data),
            error: None,
        });
    }
}

impl<T, E> Deref for UseAsyncHandle<T, E> {
    type Target = UseAsyncState<T, E>;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl<T, E> Clone for UseAsyncHandle<T, E> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            run: self.run.clone(),
        }
    }
}

impl<T, E> PartialEq for UseAsyncHandle<T, E>
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
/// use yew_hooks::prelude::*;
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
pub fn use_async<F, T, E>(future: F) -> UseAsyncHandle<T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Clone + 'static,
    E: Clone + 'static,
{
    use_async_with_options(future, UseAsyncOptions::default())
}

/// This hook returns state and a `run` callback for an async future with options.
/// See [`use_async`] too.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(Async)]
/// fn async_test() -> Html {
///     let state = use_async_with_options(async move {
///         fetch("/api/user/123".to_string()).await
///     }, UseAsyncOptions::enable_auto());
///     
///     html! {
///         <div>
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
pub fn use_async_with_options<F, T, E>(future: F, options: UseAsyncOptions) -> UseAsyncHandle<T, E>
where
    F: Future<Output = Result<T, E>> + 'static,
    T: Clone + 'static,
    E: Clone + 'static,
{
    let inner = use_state(|| UseAsyncState {
        loading: false,
        data: None,
        error: None,
    });
    let future_ref = use_mut_latest(Some(future));

    let run = {
        let inner = inner.clone();
        Rc::new(move || {
            let inner = inner.clone();
            let future_ref = future_ref.clone();
            spawn_local(async move {
                let future_ref = future_ref.current();
                let future = (*future_ref.borrow_mut()).take();

                if let Some(future) = future {
                    // Only set loading to true and leave previous data/error alone.
                    inner.set(UseAsyncState {
                        loading: true,
                        data: (*inner).data.clone(),
                        error: (*inner).error.clone(),
                    });
                    match future.await {
                        // Success with some data and clear previous error.
                        Ok(data) => inner.set(UseAsyncState {
                            loading: false,
                            data: Some(data),
                            error: None,
                        }),
                        // Failed with some error and leave previous data alone.
                        Err(error) => inner.set(UseAsyncState {
                            loading: false,
                            data: (*inner).data.clone(),
                            error: Some(error),
                        }),
                    }
                }
            });
        })
    };

    {
        let run = run.clone();
        use_mount(move || {
            if options.auto {
                run();
            }
        });
    }

    UseAsyncHandle { inner, run }
}
