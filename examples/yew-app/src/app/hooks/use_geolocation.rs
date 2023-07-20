use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_geolocation` demo
#[function_component]
pub fn UseGeolocation() -> Html {
    let state = use_geolocation();

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
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
