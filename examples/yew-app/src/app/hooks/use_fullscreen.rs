use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_fullscreen` demo
#[function_component]
pub fn UseFullscreen() -> Html {
    let element_ref = use_node_ref();
    let fullscreen = use_fullscreen(element_ref.clone());

    let onenter = {
        let fullscreen = fullscreen.clone();
        Callback::from(move |_| {
            fullscreen.enter();
        })
    };

    let onexit = {
        let fullscreen = fullscreen.clone();
        Callback::from(move |_| {
            fullscreen.exit();
        })
    };

    let ontoggle = {
        let fullscreen = fullscreen.clone();
        Callback::from(move |_| {
            fullscreen.toggle();
        })
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <div
                        ref={element_ref}
                        class="w-full h-64 bg-blue-100 border-2 border-blue-300 rounded-lg flex flex-col items-center justify-center p-4 transition-all duration-300"
                        style={if *fullscreen.is_fullscreen {
                            "background-color: #dbeafe; border-color: #60a5fa;".to_string()
                        } else {
                            "".to_string()
                        }}
                    >
                        <p class="text-lg font-semibold text-blue-800 mb-2">
                            { "Fullscreen Demo Element" }
                        </p>
                        <p class="text-blue-600 text-center">
                            { "This element can enter fullscreen mode. When in fullscreen, it will have a different background color." }
                        </p>

                        <div class="flex flex-wrap gap-4 justify-center mt-6">
                            <Button
                                onclick={onenter}
                                disabled={*fullscreen.is_fullscreen}
                            >
                                { "Enter Fullscreen" }
                            </Button>
                            <Button
                                onclick={onexit}
                                disabled={!*fullscreen.is_fullscreen}
                            >
                                { "Exit Fullscreen" }
                            </Button>
                            <Button onclick={ontoggle}>
                                { "Toggle Fullscreen" }
                            </Button>
                        </div>

                        <div class="mt-4 text-sm text-blue-500">
                            { "Try pressing F11 or ESC to exit fullscreen manually too!" }
                        </div>
                    </div>

                    <div class="mt-8 p-6 bg-gray-50 rounded-lg border border-gray-200">
                        <h3 class="text-lg font-semibold text-gray-800 mb-4">
                            { "Fullscreen State" }
                        </h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="p-4 bg-white rounded border">
                                <p class="font-medium text-gray-700">
                                    { "Is Fullscreen: " }
                                    <span class={if *fullscreen.is_fullscreen {
                                        "text-green-600 font-bold"
                                    } else {
                                        "text-red-600 font-bold"
                                    }}>
                                        { *fullscreen.is_fullscreen }
                                    </span>
                                </p>
                            </div>
                            <div class="p-4 bg-white rounded border">
                                <p class="font-medium text-gray-700">
                                    { "API Supported: " }
                                    <span class={if *fullscreen.is_supported {
                                        "text-green-600 font-bold"
                                    } else {
                                        "text-red-600 font-bold"
                                    }}>
                                        { *fullscreen.is_supported }
                                    </span>
                                </p>
                            </div>
                        </div>

                        <div class="mt-6 p-4 bg-yellow-50 rounded border border-yellow-100">
                            <h4 class="font-medium text-yellow-800 mb-2">
                                { "Note:" }
                            </h4>
                            <p class="text-yellow-700">
                                { "The Fullscreen API requires user interaction (like a button click) to work. Browsers may show a permission prompt or require specific user gestures." }
                            </p>
                            <p class="text-yellow-700 mt-2">
                                { "If 'API Supported' shows false, your browser may not support the Fullscreen API or it may be disabled." }
                            </p>
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
