use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod hooks;

use crate::components::nav::Nav;
use about::About;
use home::Home;
use hooks::*;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[at("/use_bool_toggle")]
    UseBoolToggle,
    #[at("/use_counter")]
    UseCounter,
    #[at("/use_effect_once")]
    UseEffectOnce,
    #[at("/use_interval")]
    UseInterval,
    #[at("/use_is_first_mount")]
    UseIsFirstMount,
    #[at("/use_is_mounted")]
    UseIsMounted,
    #[at("/use_mount")]
    UseMount,
    #[at("/use_timeout")]
    UseTimeout,
    #[at("/use_toggle")]
    UseToggle,
    #[at("/use_unmount")]
    UseUnmount,
    #[at("/use_update")]
    UseUpdate,
    #[at("/use_async")]
    UseAsync,
    #[at("/use_title")]
    UseTitle,
    #[at("/use_local_storage")]
    UseLocalStorage,
    #[at("/use_session_storage")]
    UseSessionStorage,
    #[at("/use_mut_latest")]
    UseMutLatest,
    #[at("/use_latest")]
    UseLatest,
    #[at("/use_previous")]
    UsePrevious,
    #[at("/use_list")]
    UseList,
    #[at("/use_map")]
    UseMap,
    #[at("/use_set")]
    UseSet,
    #[at("/use_queue")]
    UseQueue,
    #[at("/use_event")]
    UseEvent,
    #[at("/use_raf")]
    UseRaf,
    #[at("/use_raf_state")]
    UseRafState,
    #[at("/use_before_unload")]
    UseBeforeUnload,
    #[at("/use_window_size")]
    UseWindowSize,
    #[at("/use_window_scroll")]
    UseWindowScroll,
    #[at("/use_scroll")]
    UseScroll,
    #[at("/use_scrolling")]
    UseScrolling,
    #[at("/use_hash")]
    UseHash,
    #[at("/use_search_param")]
    UseSearchParam,
    #[at("/use_location")]
    UseLocation,
    #[at("/use_websocket")]
    UseWebSocket,
    #[at("/use_state_ptr_eq")]
    UseStatePtrEq,
    #[at("/use_size")]
    UseSize,
    #[at("/use_measure")]
    UseMeasure,
    #[at("/use_geolocation")]
    UseGeolocation,
    #[at("/use_click_away")]
    UseClickAway,
    #[at("/use_effect_update")]
    UseEffectUpdate,
    #[at("/use_logger")]
    UseLogger,
    #[at("/use_drag")]
    UseDrag,
    #[at("/use_drop")]
    UseDrop,
    #[at("/use_media")]
    UseMedia,
    #[at("/use_swipe")]
    UseSwipe,
    #[at("/use_renders_count")]
    UseRendersCount,
    #[at("/use_default")]
    UseDefault,
    #[at("/use_debounce")]
    UseDebounce,
    #[at("/use_debounce_state")]
    UseDebounceState,
    #[at("/use_throttle")]
    UseThrottle,
    #[at("/use_throttle_state")]
    UseThrottleState,
    #[at("/use_debounce_effect")]
    UseDebounceEffect,
    #[at("/use_throttle_effect")]
    UseThrottleEffect,
    #[at("/use_favicon")]
    UseFavicon,
    #[at("/use_clipboard")]
    UseClipboard,
    #[at("/use_infinite_scroll")]
    UseInfiniteScroll,
    #[at("/use_visible")]
    UseVisible,
    #[at("/use_virtual_list")]
    UseVirtualList,
    #[at("/use_hovered")]
    UseHovered,
    #[at("/use_permission")]
    UsePermission,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::UseBoolToggle => html! { <UseBoolToggle /> },
        AppRoute::UseCounter => html! { <UseCounter /> },
        AppRoute::UseEffectOnce => html! { <UseEffectOnce /> },
        AppRoute::UseInterval => html! { <UseInterval /> },
        AppRoute::UseIsFirstMount => html! { <UseIsFirstMount /> },
        AppRoute::UseIsMounted => html! { <UseIsMounted /> },
        AppRoute::UseMount => html! { <UseMount /> },
        AppRoute::UseTimeout => html! { <UseTimeout /> },
        AppRoute::UseToggle => html! { <UseToggle /> },
        AppRoute::UseUnmount => html! { <UseUnmount /> },
        AppRoute::UseUpdate => html! { <UseUpdate /> },
        AppRoute::UseAsync => html! { <UseAsync /> },
        AppRoute::UseTitle => html! { <UseTitle /> },
        AppRoute::UseLocalStorage => html! { <UseLocalStorage /> },
        AppRoute::UseSessionStorage => html! { <UseSessionStorage /> },
        AppRoute::UseMutLatest => html! { <UseMutLatest /> },
        AppRoute::UseLatest => html! { <UseLatest /> },
        AppRoute::UsePrevious => html! { <UsePrevious /> },
        AppRoute::UseList => html! { <UseList /> },
        AppRoute::UseMap => html! { <UseMap /> },
        AppRoute::UseSet => html! { <UseSet /> },
        AppRoute::UseQueue => html! { <UseQueue /> },
        AppRoute::UseEvent => html! { <UseEvent /> },
        AppRoute::UseRaf => html! { <UseRaf /> },
        AppRoute::UseRafState => html! { <UseRafState /> },
        AppRoute::UseBeforeUnload => html! { <UseBeforeUnload /> },
        AppRoute::UseWindowSize => html! { <UseWindowSize /> },
        AppRoute::UseWindowScroll => html! { <UseWindowScroll /> },
        AppRoute::UseScroll => html! { <UseScroll /> },
        AppRoute::UseScrolling => html! { <UseScrolling /> },
        AppRoute::UseHash => html! { <UseHash /> },
        AppRoute::UseSearchParam => html! { <UseSearchParam /> },
        AppRoute::UseLocation => html! { <UseLocation /> },
        AppRoute::UseWebSocket => html! { <UseWebSocket /> },
        AppRoute::UseStatePtrEq => html! { <UseStatePtrEq /> },
        AppRoute::UseSize => html! { <UseSize /> },
        AppRoute::UseMeasure => html! { <UseMeasure /> },
        AppRoute::UseGeolocation => html! { <UseGeolocation /> },
        AppRoute::UseClickAway => html! { <UseClickAway /> },
        AppRoute::UseEffectUpdate => html! { <UseEffectUpdate /> },
        AppRoute::UseLogger => html! { <UseLogger /> },
        AppRoute::UseDrag => html! { <UseDrag /> },
        AppRoute::UseDrop => html! { <UseDrop /> },
        AppRoute::UseMedia => html! { <UseMedia /> },
        AppRoute::UseSwipe => html! { <UseSwipe /> },
        AppRoute::UseRendersCount => html! { <UseRendersCount /> },
        AppRoute::UseDefault => html! { <UseDefault /> },
        AppRoute::UseDebounce => html! { <UseDebounce /> },
        AppRoute::UseDebounceState => html! { <UseDebounceState /> },
        AppRoute::UseThrottle => html! { <UseThrottle /> },
        AppRoute::UseThrottleState => html! { <UseThrottleState /> },
        AppRoute::UseDebounceEffect => html! { <UseDebounceEffect /> },
        AppRoute::UseThrottleEffect => html! { <UseThrottleEffect /> },
        AppRoute::UseFavicon => html! { <UseFavicon /> },
        AppRoute::UseClipboard => html! { <UseClipboard /> },
        AppRoute::UseInfiniteScroll => html! { <UseInfiniteScroll /> },
        AppRoute::UseVisible => html! { <UseVisible /> },
        AppRoute::UseVirtualList => html! { <UseVirtualList /> },
        AppRoute::UseHovered => html! { <UseHovered /> },
        AppRoute::UsePermission => html! { <UsePermission /> },
        AppRoute::PageNotFound => html! { <Home /> },
    }
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Nav />
            <Switch<AppRoute> render={switch} />
        </HashRouter>
    }
}
