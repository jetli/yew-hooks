use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_media_query` demo
#[function_component]
pub fn UseMediaQuery() -> Html {
    let is_small_screen = use_media_query("(max-width: 600px)");
    let is_medium_screen = use_media_query("(min-width: 601px) and (max-width: 1024px)");
    let is_large_screen = use_media_query("(min-width: 1025px)");
    let is_dark_mode = use_media_query("(prefers-color-scheme: dark)");
    let is_light_mode = use_media_query("(prefers-color-scheme: light)");
    let is_reduced_motion = use_media_query("(prefers-reduced-motion: reduce)");
    let is_high_contrast = use_media_query("(prefers-contrast: high)");
    let is_portrait = use_media_query("(orientation: portrait)");
    let is_landscape = use_media_query("(orientation: landscape)");

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-y-6 max-w-2xl mx-auto">
                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <div class="text-left space-y-3">
                            <p>{ "Try resizing your browser window or changing your system's color scheme to see the hook in action!" }</p>
                        </div>
                    </div>
                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Screen Size" }</h2>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <div class={format!("p-3 rounded {}", if is_small_screen { "bg-green-200 dark:bg-green-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Small Screen" }</p>
                                <p class="text-sm">{ "(max-width: 600px)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_small_screen { "text-green-700 dark:text-green-300" } else { "text-gray-500" })}>
                                    { if is_small_screen { "✓ Matches" } else { "✗ No match" } }
                                </p>
                            </div>
                            <div class={format!("p-3 rounded {}", if is_medium_screen { "bg-green-200 dark:bg-green-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Medium Screen" }</p>
                                <p class="text-sm">{ "(601px - 1024px)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_medium_screen { "text-green-700 dark:text-green-300" } else { "text-gray-500" })}>
                                    { if is_medium_screen { "✓ Matches" } else { "✗ No match" } }
                                </p>
                            </div>
                            <div class={format!("p-3 rounded {}", if is_large_screen { "bg-green-200 dark:bg-green-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Large Screen" }</p>
                                <p class="text-sm">{ "(min-width: 1025px)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_large_screen { "text-green-700 dark:text-green-300" } else { "text-gray-500" })}>
                                    { if is_large_screen { "✓ Matches" } else { "✗ No match" } }
                                </p>
                            </div>
                        </div>
                    </div>

                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Color Scheme" }</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class={format!("p-3 rounded {}", if is_dark_mode { "bg-purple-200 dark:bg-purple-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Dark Mode" }</p>
                                <p class="text-sm">{ "(prefers-color-scheme: dark)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_dark_mode { "text-purple-700 dark:text-purple-300" } else { "text-gray-500" })}>
                                    { if is_dark_mode { "✓ Preferred" } else { "✗ Not preferred" } }
                                </p>
                            </div>
                            <div class={format!("p-3 rounded {}", if is_light_mode { "bg-yellow-200 dark:bg-yellow-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Light Mode" }</p>
                                <p class="text-sm">{ "(prefers-color-scheme: light)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_light_mode { "text-yellow-700 dark:text-yellow-300" } else { "text-gray-500" })}>
                                    { if is_light_mode { "✓ Preferred" } else { "✗ Not preferred" } }
                                </p>
                            </div>
                        </div>
                    </div>

                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Accessibility" }</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class={format!("p-3 rounded {}", if is_reduced_motion { "bg-blue-200 dark:bg-blue-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Reduced Motion" }</p>
                                <p class="text-sm">{ "(prefers-reduced-motion: reduce)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_reduced_motion { "text-blue-700 dark:text-blue-300" } else { "text-gray-500" })}>
                                    { if is_reduced_motion { "✓ Preferred" } else { "✗ Not preferred" } }
                                </p>
                            </div>
                            <div class={format!("p-3 rounded {}", if is_high_contrast { "bg-red-200 dark:bg-red-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "High Contrast" }</p>
                                <p class="text-sm">{ "(prefers-contrast: high)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_high_contrast { "text-red-700 dark:text-red-300" } else { "text-gray-500" })}>
                                    { if is_high_contrast { "✓ Preferred" } else { "✗ Not preferred" } }
                                </p>
                            </div>
                        </div>
                    </div>

                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Orientation" }</h2>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class={format!("p-3 rounded {}", if is_portrait { "bg-indigo-200 dark:bg-indigo-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Portrait" }</p>
                                <p class="text-sm">{ "(orientation: portrait)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_portrait { "text-indigo-700 dark:text-indigo-300" } else { "text-gray-500" })}>
                                    { if is_portrait { "✓ Current" } else { "✗ Not current" } }
                                </p>
                            </div>
                            <div class={format!("p-3 rounded {}", if is_landscape { "bg-pink-200 dark:bg-pink-800" } else { "bg-gray-200 dark:bg-gray-700" })}>
                                <p class="font-medium">{ "Landscape" }</p>
                                <p class="text-sm">{ "(orientation: landscape)" }</p>
                                <p class={format!("text-lg font-bold {}", if is_landscape { "text-pink-700 dark:text-pink-300" } else { "text-gray-500" })}>
                                    { if is_landscape { "✓ Current" } else { "✗ Not current" } }
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
