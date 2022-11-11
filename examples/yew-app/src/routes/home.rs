use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="app">
            <header class="app-header">
                <h1>{ "Yew Hooks" }</h1>
                <div class="hooks">
                    <h2>{ "State" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseToggle} classes="app-link" >{ "use_toggle" }</Link<AppRoute>> { " - tracks state of counterparts." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseBoolToggle} classes="app-link">{ "use_bool_toggle" }</Link<AppRoute>> { " - tracks state of a boolean." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseCounter} classes="app-link">{ "use_counter" }</Link<AppRoute>> { " - tracks state of a number." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLatest} classes="app-link">{ "use_latest" }</Link<AppRoute>> { " - returns the latest immutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMutLatest} classes="app-link">{ "use_mut_latest" }</Link<AppRoute>> { " - returns the latest mutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UsePrevious} classes="app-link">{ "use_previous" }</Link<AppRoute>> { " - returns the previous immutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseList} classes="app-link">{ "use_list" }</Link<AppRoute>> { " - tracks state of a list." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMap} classes="app-link">{ "use_map" }</Link<AppRoute>> { " - tracks state of a hash map." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSet} classes="app-link">{ "use_set" }</Link<AppRoute>> { " - tracks state of a hash set." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseQueue} classes="app-link">{ "use_queue" }</Link<AppRoute>> { " - tracks state of a queue." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRafState} classes="app-link">{ "use_raf_state" }</Link<AppRoute>> { " - creates set method which only updates after requestAnimationFrame." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseStatePtrEq} classes="app-link">{ "use_state_ptr_eq" }</Link<AppRoute>> { " - similar to use_state_eq, but checks two Rcs' pointers of allocation." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRendersCount} classes="app-link">{ "use_renders_count" }</Link<AppRoute>> { " - counts component renders." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDefault} classes="app-link">{ "use_default" }</Link<AppRoute>> { " - returns the default value when state is None." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDebounceState} classes="app-link">{ "use_debounce_state" }</Link<AppRoute>> { " - debounces state." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseThrottleState} classes="app-link">{ "use_throttle_state" }</Link<AppRoute>> { " - throttles state." }</li>
                    </ul>

                    <h2>{ "Side-effects" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseAsync} classes="app-link" >{ "use_async" }</Link<AppRoute>> { " - resolves an async future, e.g. fetching REST api." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseWebSocket} classes="app-link" >{ "use_websocket" }</Link<AppRoute>> { " - communicates with WebSocket." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseTitle} classes="app-link" >{ "use_title" }</Link<AppRoute>> { " - sets title of the page." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseFavicon} classes="app-link" >{ "use_favicon" }</Link<AppRoute>> { " - sets favicon of the page." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLocalStorage} classes="app-link" >{ "use_local_storage" }</Link<AppRoute>> { " - manages a value in localStorage." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSessionStorage} classes="app-link" >{ "use_session_storage" }</Link<AppRoute>> { " - manages a value in sessionStorage." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseBeforeUnload} classes="app-link" >{ "use_before_unload" }</Link<AppRoute>> { " - shows browser alert when user try to reload or close the page." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDebounce} classes="app-link" >{ "use_debounce" }</Link<AppRoute>> { " - debounces a function." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDebounceEffect} classes="app-link" >{ "use_debounce_effect" }</Link<AppRoute>> { " - debounces an effect." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseThrottle} classes="app-link" >{ "use_throttle" }</Link<AppRoute>> { " - throttles a function." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseThrottleEffect} classes="app-link" >{ "use_throttle_effect" }</Link<AppRoute>> { " - throttles an effect." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseClipboard} classes="app-link" >{ "use_clipboard" }</Link<AppRoute>> { " - reads from or writes to clipboard for text/bytes." }</li>
                    </ul>

                    <h2>{ "Lifecycles" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseEffectOnce} classes="app-link" >{ "use_effect_once" }</Link<AppRoute>> { " - a modified use_effect hook that only runs once." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMount} classes="app-link">{ "use_mount" }</Link<AppRoute>> { " - calls mount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUnmount} classes="app-link">{ "use_unmount" }</Link<AppRoute>> { " - calls unmount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsFirstMount} classes="app-link">{ "use_is_first_mount" }</Link<AppRoute>> { " - checks if current render is first." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsMounted} classes="app-link">{ "use_is_mounted" }</Link<AppRoute>> { " - tracks if component is mounted." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseEvent} classes="app-link">{ "use_event" }</Link<AppRoute>> { " - subscribe to events." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseEffectUpdate} classes="app-link">{ "use_effect_update" }</Link<AppRoute>> { " - runs an effect only on updates." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLogger} classes="app-link">{ "use_logger" }</Link<AppRoute>> { " - logs in console as component goes through life cycles." }</li>
                    </ul>

                    <h2>{ "Animations" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseTimeout} classes="app-link" >{ "use_timeout" }</Link<AppRoute>> { " - schedules a timeout to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseInterval} classes="app-link">{ "use_interval" }</Link<AppRoute>> { " - schedules an interval to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUpdate} classes="app-link">{ "use_update" }</Link<AppRoute>> { " - returns a callback, which re-renders component when called." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRaf} classes="app-link">{ "use_raf" }</Link<AppRoute>> { " - re-renders component on each requestAnimationFrame." }</li>
                    </ul>

                    <h2>{ "Sensors" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseWindowSize} classes="app-link" >{ "use_window_size" }</Link<AppRoute>> { " - tracks Window dimensions." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseWindowScroll} classes="app-link" >{ "use_window_scroll" }</Link<AppRoute>> { " - tracks Window scroll position." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseScroll} classes="app-link" >{ "use_scroll" }</Link<AppRoute>> { " - tracks an HTML element's scroll position." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseScrolling} classes="app-link" >{ "use_scrolling" }</Link<AppRoute>> { " - tracks whether HTML element is scrolling." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseInfiniteScroll} classes="app-link" >{ "use_infinite_scroll" }</Link<AppRoute>> { " - infinite scrolling of the element." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLocation} classes="app-link" >{ "use_location" }</Link<AppRoute>> { " - tracks brower's location value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseHash} classes="app-link" >{ "use_hash" }</Link<AppRoute>> { " - tracks brower's location hash value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSearchParam} classes="app-link" >{ "use_search_param" }</Link<AppRoute>> { " - tracks brower's location search param value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSize} classes="app-link" >{ "use_size" }</Link<AppRoute>> { " - tracks an HTML element's dimensions using the ResizeObserver API." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMeasure} classes="app-link" >{ "use_measure" }</Link<AppRoute>> { " - tracks an HTML element's dimensions using the ResizeObserver API." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseGeolocation} classes="app-link" >{ "use_geolocation" }</Link<AppRoute>> { " - tracks user's geographic location." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSwipe} classes="app-link" >{ "use_swipe" }</Link<AppRoute>> { " - detects swipe based on TouchEvent." }</li>
                    </ul>

                    <h2>{ "UI" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseClickAway} classes="app-link" >{ "use_click_away" }</Link<AppRoute>> { " - triggers a callback when user clicks outside the target element." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDrag} classes="app-link" >{ "use_drag" }</Link<AppRoute>> { " - tracks file, link and copy-paste drags, used along with use_drop hook." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDrop} classes="app-link" >{ "use_drop" }</Link<AppRoute>> { " - tracks file, link and copy-paste drops." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMedia} classes="app-link" >{ "use_media" }</Link<AppRoute>> { " - plays video or audio and exposes its controls." }</li>
                    </ul>
                </div>
                <p>
                    { "More is coming.." }
                </p>
                <a
                    class="app-logo"
                    href="https://yew.rs"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                </a>
            </header>
        </div>
    }
}
