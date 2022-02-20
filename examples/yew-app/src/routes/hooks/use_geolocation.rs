use yew::prelude::*;

use yew_hooks::use_geolocation;

/// `use_geolocation` demo
#[function_component(UseGeolocation)]
pub fn geolocation() -> Html {
    let state = use_geolocation();

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ " loading: " }</b>
                        { state.loading }
                        <b>{ " latitude: " }</b>
                        { state.latitude }
                        <b>{ " longitude: " }</b>
                        { state.longitude }
                        <b>{ " altitude: " }</b>
                        { state.altitude.unwrap_or_default() }
                        <b>{ " altitude_accuracy: " }</b>
                        { state.altitude_accuracy.unwrap_or_default() }
                        <b>{ " heading: " }</b>
                        { state.heading.unwrap_or_default() }
                        <b>{ " speed: " }</b>
                        { state.speed.unwrap_or_default() }
                        <b>{ " timestamp: " }</b>
                        { state.timestamp }
                        <b>{ " error: " }</b>
                        { &format!("{:?}", state.error) }
                    </p>
                </div>
            </header>
        </div>
    }
}
