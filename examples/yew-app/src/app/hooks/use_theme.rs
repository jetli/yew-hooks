use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_theme` demo page
#[function_component]
pub fn UseTheme() -> Html {
    // Use a namespaced key so it doesn't collide with other examples
    let theme = use_theme("yew_app_theme".to_string());

    let on_toggle = {
        let theme = theme.clone();
        Callback::from(move |_| theme.toggle())
    };

    let on_set_dark = {
        let theme = theme.clone();
        Callback::from(move |_| theme.set_dark())
    };

    let on_set_light = {
        let theme = theme.clone();
        Callback::from(move |_| theme.set_light())
    };

    let on_set_system = {
        let theme = theme.clone();
        Callback::from(move |_| theme.set_system())
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center space-y-6">
                <div class="space-x-4">
                    <Button onclick={on_toggle}>{ "Toggle" }</Button>
                    <Button onclick={on_set_dark}>{ "Dark" }</Button>
                    <Button onclick={on_set_light}>{ "Light" }</Button>
                    <Button onclick={on_set_system}>{ "Use system" }</Button>
                </div>

                <p>
                    <b>{ "Current theme: " }</b>
                    { (*theme).clone() }
                    { " " }
                    {
                        if theme.is_dark() {
                            html! { <span class="text-sm text-gray-500">{"(dark active)"}</span> }
                        } else {
                            html! { <span class="text-sm text-gray-500">{"(light active)"}</span> }
                        }
                    }
                </p>

                <div class="max-w-md mx-auto">
                    <div class="p-6 rounded shadow transition-colors duration-200
                                bg-white text-black
                                dark:bg-gray-800 dark:text-white">
                        <h3 class="text-lg font-semibold">{ "Preview box" }</h3>
                        <p class="mt-2 text-sm">
                            { "This box demonstrates applying the theme to the document element. Toggle theme to see the dark mode styles (Tailwind's `dark:` classes) take effect." }
                        </p>
                    </div>
                </div>

                <p class="text-sm text-gray-500">
                    { "Note: the example persists your explicit choice to localStorage under the key " }
                    <code>{ "yew_app_theme" }</code>
                    { ". If you choose 'Use system' the page will follow your OS/browser color scheme." }
                </p>
            </header>
        </div>
    }
}
