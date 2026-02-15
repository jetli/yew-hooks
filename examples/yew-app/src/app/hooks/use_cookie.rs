use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// A component that demonstrates the use_cookie hook
#[function_component]
pub fn UseCookie() -> Html {
    let cookie = use_cookie::<String>("example_cookie".to_string());

    let on_set_string = {
        let cookie = cookie.clone();
        Callback::from(move |_| {
            cookie.set("Hello from Yew Hooks!".to_string());
        })
    };

    let on_set_number = {
        let cookie = cookie.clone();
        Callback::from(move |_| {
            cookie.set("42".to_string());
        })
    };

    let on_set_json = {
        let cookie = cookie.clone();
        Callback::from(move |_| {
            // For JSON, we need to serialize it to a string
            let json_string = r#"{"name":"Yew Hooks","version":"0.4.4","feature":"use_cookie"}"#;
            cookie.set(json_string.to_string());
        })
    };

    let on_set_with_attributes = {
        let cookie = cookie.clone();
        Callback::from(move |_| {
            use yew_hooks::{CookieAttributes, SameSite};

            // Set cookie to expire in 1 hour (using max_age)
            let attributes = CookieAttributes {
                max_age: Some(3600), // 1 hour in seconds
                path: Some("/".to_string()),
                secure: false, // Set to true in production with HTTPS
                same_site: Some(SameSite::Lax),
                ..Default::default()
            };

            cookie.set_with_attributes("Cookie with attributes".to_string(), attributes);
        })
    };

    let on_delete = {
        let cookie = cookie.clone();
        Callback::from(move |_| {
            cookie.delete();
        })
    };

    html! {
        <div class="container mx-auto px-4">
            <div class="max-w-4xl mx-auto">
                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-8">
                    <h2 class="text-2xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
                        { "Current Cookie Value" }
                    </h2>
                    <div class="bg-gray-50 dark:bg-gray-900 rounded-lg p-4 border border-gray-200 dark:border-gray-700">
                        {
                            if let Some(value) = &*cookie {
                                html! {
                                    <code class="text-green-600 dark:text-green-400 font-mono text-lg break-all">
                                        { value }
                                    </code>
                                }
                            } else {
                                html! {
                                    <em class="text-gray-500 dark:text-gray-400 italic">
                                        { "No cookie set" }
                                    </em>
                                }
                            }
                        }
                    </div>
                </div>

                <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 dark:text-gray-100 mb-6">
                        { "Cookie Operations" }
                    </h2>
                    <div class="flex flex-wrap gap-4">
                        <Button onclick={on_set_string}>
                            { "Set String Cookie" }
                        </Button>
                        <Button onclick={on_set_number}>
                            { "Set Number Cookie" }
                        </Button>
                        <Button onclick={on_set_json}>
                            { "Set JSON Cookie" }
                        </Button>
                        <Button onclick={on_set_with_attributes}>
                            { "Set Cookie with Attributes" }
                        </Button>
                        <Button onclick={on_delete}>
                            { "Delete Cookie" }
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}
