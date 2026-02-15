use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <header class="space-y-8 w-[800px] mx-auto text-xl">
                <a href="https://yew.rs" target="_blank" rel="noopener noreferrer">
                    <img class="w-48 h-48 mt-24 mx-auto" src="logo.svg" alt="Yew" />
                </a>
                <h1 id="yew_hooks" class="text-4xl font-bold text-center mb-18">{ "Yew Hooks" }</h1>
                <div class="space-y-4">
                    <h2 class="text-2xl font-bold">{ "State" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseToggle} classes="text-emerald-800 underline" >{ "use_toggle" }</Link<AppRoute>> { " - tracks state of counterparts." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseBoolToggle} classes="text-emerald-800 underline">{ "use_bool_toggle" }</Link<AppRoute>> { " - tracks state of a boolean." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseCounter} classes="text-emerald-800 underline">{ "use_counter" }</Link<AppRoute>> { " - tracks state of a number." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLatest} classes="text-emerald-800 underline">{ "use_latest" }</Link<AppRoute>> { " - returns the latest immutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMutLatest} classes="text-emerald-800 underline">{ "use_mut_latest" }</Link<AppRoute>> { " - returns the latest mutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UsePrevious} classes="text-emerald-800 underline">{ "use_previous" }</Link<AppRoute>> { " - returns the previous immutable ref to state or props." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseList} classes="text-emerald-800 underline">{ "use_list" }</Link<AppRoute>> { " - tracks state of a list." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseVirtualList} classes="text-emerald-800 underline">{ "use_virtual_list" }</Link<AppRoute>> { " - provides virtual scrolling for large lists." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMap} classes="text-emerald-800 underline">{ "use_map" }</Link<AppRoute>> { " - tracks state of a hash map." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSet} classes="text-emerald-800 underline">{ "use_set" }</Link<AppRoute>> { " - tracks state of a hash set." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseQueue} classes="text-emerald-800 underline">{ "use_queue" }</Link<AppRoute>> { " - tracks state of a queue." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRafState} classes="text-emerald-800 underline">{ "use_raf_state" }</Link<AppRoute>> { " - creates set method which only updates after requestAnimationFrame." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseStatePtrEq} classes="text-emerald-800 underline">{ "use_state_ptr_eq" }</Link<AppRoute>> { " - similar to use_state_eq, but checks two Rcs' pointers of allocation." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRendersCount} classes="text-emerald-800 underline">{ "use_renders_count" }</Link<AppRoute>> { " - counts component renders." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDefault} classes="text-emerald-800 underline">{ "use_default" }</Link<AppRoute>> { " - returns the default value when state is None." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDebounceState} classes="text-emerald-800 underline">{ "use_debounce_state" }</Link<AppRoute>> { " - debounces state." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseThrottleState} classes="text-emerald-800 underline">{ "use_throttle_state" }</Link<AppRoute>> { " - throttles state." }</li>
                    </ul>

                    <h2 class="text-2xl font-bold">{ "Side-effects" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseAsync} classes="text-emerald-800 underline" >{ "use_async" }</Link<AppRoute>> { " - resolves an async future, e.g. fetching REST api." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseWebSocket} classes="text-emerald-800 underline" >{ "use_websocket" }</Link<AppRoute>> { " - communicates with WebSocket." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseTitle} classes="text-emerald-800 underline" >{ "use_title" }</Link<AppRoute>> { " - sets title of the page." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseFavicon} classes="text-emerald-800 underline" >{ "use_favicon" }</Link<AppRoute>> { " - sets favicon of the page." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLocalStorage} classes="text-emerald-800 underline" >{ "use_local_storage" }</Link<AppRoute>> { " - manages a value in localStorage." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSessionStorage} classes="text-emerald-800 underline" >{ "use_session_storage" }</Link<AppRoute>> { " - manages a value in sessionStorage." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseCookie} classes="text-emerald-800 underline" >{ "use_cookie" }</Link<AppRoute>> { " - manages browser cookies." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseBeforeUnload} classes="text-emerald-800 underline" >{ "use_before_unload" }</Link<AppRoute>> { " - shows browser alert when user try to reload or close the page." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDebounce} classes="text-emerald-800 underline" >{ "use_debounce" }</Link<AppRoute>> { " - debounces a function." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDebounceEffect} classes="text-emerald-800 underline" >{ "use_debounce_effect" }</Link<AppRoute>> { " - debounces an effect." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseThrottle} classes="text-emerald-800 underline" >{ "use_throttle" }</Link<AppRoute>> { " - throttles a function." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseThrottleEffect} classes="text-emerald-800 underline" >{ "use_throttle_effect" }</Link<AppRoute>> { " - throttles an effect." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseClipboard} classes="text-emerald-800 underline" >{ "use_clipboard" }</Link<AppRoute>> { " - reads from or writes to clipboard for text/bytes." }</li>
                    </ul>

                    <h2 class="text-2xl font-bold">{ "Lifecycles" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseEffectOnce} classes="text-emerald-800 underline" >{ "use_effect_once" }</Link<AppRoute>> { " - a modified use_effect hook that only runs once." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMount} classes="text-emerald-800 underline">{ "use_mount" }</Link<AppRoute>> { " - calls mount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUnmount} classes="text-emerald-800 underline">{ "use_unmount" }</Link<AppRoute>> { " - calls unmount callbacks." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsFirstMount} classes="text-emerald-800 underline">{ "use_is_first_mount" }</Link<AppRoute>> { " - checks if current render is first." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseIsMounted} classes="text-emerald-800 underline">{ "use_is_mounted" }</Link<AppRoute>> { " - tracks if component is mounted." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseEvent} classes="text-emerald-800 underline">{ "use_event" }</Link<AppRoute>> { " - subscribe to events." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseEffectUpdate} classes="text-emerald-800 underline">{ "use_effect_update" }</Link<AppRoute>> { " - runs an effect only on updates." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLogger} classes="text-emerald-800 underline">{ "use_logger" }</Link<AppRoute>> { " - logs in console as component goes through life cycles." }</li>
                    </ul>

                    <h2 class="text-2xl font-bold">{ "Animations" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseTimeout} classes="text-emerald-800 underline" >{ "use_timeout" }</Link<AppRoute>> { " - schedules a timeout to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseInterval} classes="text-emerald-800 underline">{ "use_interval" }</Link<AppRoute>> { " - schedules an interval to invoke callback." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseUpdate} classes="text-emerald-800 underline">{ "use_update" }</Link<AppRoute>> { " - returns a callback, which re-renders component when called." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseRaf} classes="text-emerald-800 underline">{ "use_raf" }</Link<AppRoute>> { " - re-renders component on each requestAnimationFrame." }</li>
                    </ul>

                    <h2 class="text-2xl font-bold">{ "Sensors" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseWindowSize} classes="text-emerald-800 underline" >{ "use_window_size" }</Link<AppRoute>> { " - tracks Window dimensions." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseWindowScroll} classes="text-emerald-800 underline" >{ "use_window_scroll" }</Link<AppRoute>> { " - tracks Window scroll position." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseScroll} classes="text-emerald-800 underline" >{ "use_scroll" }</Link<AppRoute>> { " - tracks an HTML element's scroll position." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseScrolling} classes="text-emerald-800 underline" >{ "use_scrolling" }</Link<AppRoute>> { " - tracks whether HTML element is scrolling." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseInfiniteScroll} classes="text-emerald-800 underline" >{ "use_infinite_scroll" }</Link<AppRoute>> { " - infinite scrolling of the element." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseLocation} classes="text-emerald-800 underline" >{ "use_location" }</Link<AppRoute>> { " - tracks brower's location value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseHash} classes="text-emerald-800 underline" >{ "use_hash" }</Link<AppRoute>> { " - tracks brower's location hash value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSearchParam} classes="text-emerald-800 underline" >{ "use_search_param" }</Link<AppRoute>> { " - tracks brower's location search param value." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSize} classes="text-emerald-800 underline" >{ "use_size" }</Link<AppRoute>> { " - tracks an HTML element's dimensions using the ResizeObserver API." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMeasure} classes="text-emerald-800 underline" >{ "use_measure" }</Link<AppRoute>> { " - tracks an HTML element's dimensions using the ResizeObserver API." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseGeolocation} classes="text-emerald-800 underline" >{ "use_geolocation" }</Link<AppRoute>> { " - tracks user's geographic location." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseSwipe} classes="text-emerald-800 underline" >{ "use_swipe" }</Link<AppRoute>> { " - detects swipe based on TouchEvent." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseVisible} classes="text-emerald-800 underline" >{ "use_visible" }</Link<AppRoute>> { " - checks if an element is visible." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseHovered} classes="text-emerald-800 underline" >{ "use_hovered" }</Link<AppRoute>> { " - checks if an element is being hovered." }</li>
                        <li><Link<AppRoute> to={AppRoute::UsePermission} classes="text-emerald-800 underline" >{ "use_permission" }</Link<AppRoute>> { " - tracks browser's permission changes using the Permissions API." }</li>
                    </ul>

                    <h2 class="text-2xl font-bold">{ "UI" }</h2>

                    <ul>
                        <li><Link<AppRoute> to={AppRoute::UseClickAway} classes="text-emerald-800 underline" >{ "use_click_away" }</Link<AppRoute>> { " - triggers a callback when user clicks outside the target element." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDrag} classes="text-emerald-800 underline" >{ "use_drag" }</Link<AppRoute>> { " - tracks file, link and copy-paste drags, used along with use_drop hook." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseDrop} classes="text-emerald-800 underline" >{ "use_drop" }</Link<AppRoute>> { " - tracks file, link and copy-paste drops." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseMedia} classes="text-emerald-800 underline" >{ "use_media" }</Link<AppRoute>> { " - plays video or audio and exposes its controls." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseFullscreen} classes="text-emerald-800 underline" >{ "use_fullscreen" }</Link<AppRoute>> { " - controls fullscreen mode for elements." }</li>
                        <li><Link<AppRoute> to={AppRoute::UseTheme} classes="text-emerald-800 underline" >{ "use_theme" }</Link<AppRoute>> { " - toggles site light/dark theme with persistence." }</li>
                    </ul>
                </div>
                <p>
                    { "More is coming.." }
                </p>
            </header>
        </div>
    }
}
