use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav>
            <ul>
                <li><Link<AppRoute> to={AppRoute::Home} classes="app-link" >{ "Home" }</Link<AppRoute>></li>
                <li><Link<AppRoute> to={AppRoute::About} classes="app-link">{ "About" }</Link<AppRoute>></li>
            </ul>
        </nav>
    }
}
