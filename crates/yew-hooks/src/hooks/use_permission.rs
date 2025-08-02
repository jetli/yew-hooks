use gloo::utils::window;
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{PermissionState, PermissionStatus};
use yew::prelude::*;

use crate::use_effect_once;

/// A sensor hook that tracks browser's permission changes
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UsePermission)]
/// fn permission() -> Html {
///     let state = use_permission("notifications".to_owned());
///
///     html! {
///         <>
///             <b>{ " state: " }</b>
///             { format!("{state:?}") }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_permission(name: String) -> UseStateHandle<Option<PermissionState>> {
    let state = use_state(Option::<PermissionState>::default);

    {
        let state = state.clone();

        use_effect_once(move || {
            let state = state.clone();

            spawn_local(async move {
                let permissions = window().navigator().permissions().unwrap_throw();

                let obj = Object::new();
                Reflect::set(&obj, &"name".into(), &name.into()).unwrap_throw();

                let fut = JsFuture::from(permissions.query(&obj).unwrap_throw());
                let stat = PermissionStatus::from(fut.await.unwrap_throw());

                let onchange = {
                    let state = state.clone();

                    Closure::wrap(Box::new(move |event: Event| {
                        if let Some(target) = event.target() {
                            let stat: PermissionStatus = target.dyn_into().unwrap_throw();

                            state.set(Some(stat.state()));
                        }
                    }) as Box<dyn FnMut(Event)>)
                };

                stat.set_onchange(Some(onchange.as_ref().unchecked_ref()));
                onchange.forget();

                state.set(Some(stat.state()));
            });

            move || {}
        });
    }

    state
}
