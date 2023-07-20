use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <div class="container py-4">
            <nav class="flex space-x-4 items-center">
                <Link<AppRoute> to={AppRoute::Home} classes="text-emerald-800 underline" >
                    <img class="w-10 h-10" src="logo.svg" alt="Yew" />
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Home} classes="text-emerald-800 underline" >{ "Home" }</Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::About} classes="text-emerald-800 underline">{ "About" }</Link<AppRoute>>
                <a class="text-emerald-800 underline" href="https://github.com/jetli/yew-hooks" target="_blank">{ "Github" }</a>
                <a class="text-emerald-800 underline" href="https://docs.rs/yew-hooks/" target="_blank">{ "Docs" }</a>
            </nav>
        </div>
    }
}
