use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod hooks;

use about::About;
use home::Home;
use hooks::*;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
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
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
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
        AppRoute::PageNotFound => html! { <Home /> },
    }
}
