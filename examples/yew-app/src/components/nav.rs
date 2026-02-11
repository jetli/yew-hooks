use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    // Use the same storage key as the example pages so the toggle stays in sync.
    let theme = use_theme("yew_app_theme".to_string());
    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_| theme.toggle())
    };

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

               <div class="flex-1" />

               <button
                   {onclick}
                   class="inline-flex items-center justify-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-emerald-400"
                   aria-label={ if theme.is_dark() { "Switch to light theme" } else { "Switch to dark theme" } }
               >
                   {
                       if theme.is_dark() {
                           // Show sun icon when dark (click to go to light)
                           html! {
                               <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-yellow-400" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                   <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v2m0 14v2m9-9h-2M5 12H3m15.364-6.364l-1.414 1.414M7.05 16.95l-1.414 1.414M18.364 18.364l-1.414-1.414M7.05 7.05L5.636 5.636M12 7a5 5 0 100 10 5 5 0 000-10z"/>
                               </svg>
                           }
                       } else {
                           // Show moon icon when light (click to go to dark)
                           html! {
                               <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-gray-700 dark:text-gray-200" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                   <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z"/>
                               </svg>
                           }
                       }
                   }
               </button>
            </nav>
        </div>
    }
}
