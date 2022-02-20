use gloo::utils::window;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Position, PositionError};

use yew::prelude::*;

use super::use_effect_once;

pub use web_sys::PositionOptions as UseGeolocationOptions;

#[derive(PartialEq, Default, Clone)]
pub struct UseGeolocationState {
    pub loading: bool,
    pub accuracy: f64,
    pub altitude: Option<f64>,
    pub altitude_accuracy: Option<f64>,
    pub heading: Option<f64>,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: Option<f64>,
    pub timestamp: f64,
    pub error: Option<PositionError>,
}

/// A sensor hook that tracks user's geographic location.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_geolocation;
///
/// #[function_component(UseGeolocation)]
/// fn geolocation() -> Html {
///     let state = use_geolocation();
///     
///     html! {
///         <>
///             <b>{ " loading: " }</b>
///             { state.loading }
///             <b>{ " latitude: " }</b>
///             { state.latitude }
///             <b>{ " longitude: " }</b>
///             { state.longitude }
///             <b>{ " altitude: " }</b>
///             { state.altitude.unwrap_or_default() }
///             <b>{ " altitude_accuracy: " }</b>
///             { state.altitude_accuracy.unwrap_or_default() }
///             <b>{ " heading: " }</b>
///             { state.heading.unwrap_or_default() }
///             <b>{ " speed: " }</b>
///             { state.speed.unwrap_or_default() }
///             <b>{ " timestamp: " }</b>
///             { state.timestamp }
///         </>
///     }
/// }
/// ```
pub fn use_geolocation() -> UseGeolocationState {
    use_geolocation_with_options(UseGeolocationOptions::default())
}

/// A sensor hook that tracks user's geographic location.
/// See [`use_geolocation`]
pub fn use_geolocation_with_options(options: UseGeolocationOptions) -> UseGeolocationState {
    let state = use_state(|| UseGeolocationState {
        loading: true,
        ..Default::default()
    });

    {
        let state = state.clone();
        use_effect_once(move || {
            let closure = {
                let state = state.clone();
                Closure::wrap(Box::new(move |position: Position| {
                    state.set(UseGeolocationState {
                        loading: false,
                        accuracy: position.coords().accuracy(),
                        altitude: position.coords().altitude(),
                        altitude_accuracy: position.coords().altitude_accuracy(),
                        heading: position.coords().heading(),
                        latitude: position.coords().latitude(),
                        longitude: position.coords().longitude(),
                        speed: position.coords().speed(),
                        timestamp: position.timestamp(),
                        error: None,
                    });
                }) as Box<dyn Fn(Position)>)
            };

            let error_closure = {
                let state = state.clone();
                Closure::wrap(Box::new(move |error: PositionError| {
                    state.set(UseGeolocationState {
                        loading: false,
                        error: Some(error),
                        ..*state
                    });
                }) as Box<dyn Fn(PositionError)>)
            };

            window()
                .navigator()
                .geolocation()
                .unwrap_throw()
                .get_current_position_with_error_callback_and_options(
                    closure.as_ref().unchecked_ref(),
                    Some(error_closure.as_ref().unchecked_ref()),
                    &options,
                )
                .unwrap_throw();

            let watch_id = window()
                .navigator()
                .geolocation()
                .unwrap_throw()
                .watch_position_with_error_callback_and_options(
                    closure.as_ref().unchecked_ref(),
                    Some(error_closure.as_ref().unchecked_ref()),
                    &options,
                )
                .unwrap_throw();

            // Forget the closure to keep it alive
            closure.forget();
            error_closure.forget();

            move || {
                window()
                    .navigator()
                    .geolocation()
                    .unwrap_throw()
                    .clear_watch(watch_id)
            }
        });
    }

    (*state).clone()
}
