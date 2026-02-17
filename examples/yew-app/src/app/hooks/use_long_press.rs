use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// The `UseLongPress` component demonstrates the `use_long_press` hook.
/// It shows how to detect when a user presses and holds an element for a specified duration.
#[function_component]
pub fn UseLongPress() -> Html {
    let button1_ref = use_node_ref();
    let button2_ref = use_node_ref();
    let duration = 1000; // 1 second threshold
    let move_threshold = 50.0; // 50 pixels movement threshold

    // Example 1: Basic long press without move threshold
    let long_press1 = use_long_press(button1_ref.clone(), duration, move |_| {
        // Long press callback
    });

    // Example 2: Long press with move threshold
    let long_press2 = use_long_press_with_options(
        button2_ref.clone(),
        yew_hooks::UseLongPressOptions {
            threshold: Some(duration),
            move_threshold: Some(move_threshold),
            onlongpress: None,
            onstart: None,
            onend: None,
            prevent_default: None,
        },
    );

    html! {
        <div class="flex flex-col items-center justify-center p-8 space-y-8">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8 w-full max-w-4xl">

                <div class="bg-white rounded-xl shadow-lg p-6 space-y-6">
                    <h3 class="text-xl font-bold text-gray-800 mb-4 text-center">
                        {"Basic Long Press"}
                    </h3>

                    <div class="text-center space-y-4">
                        <Button
                            button_ref={button1_ref.clone()}
                            class="w-full"
                            onclick={Callback::from(|_| {})}
                        >
                            {"Press and hold for "}{duration}{"ms"}
                        </Button>

                        <div class="space-y-2">
                            <div class="text-lg font-medium text-gray-800">
                                {"Status: "}
                                <span class="font-bold">
                                    {if *long_press1.long_pressed {
                                        "Long pressed!"
                                    } else if *long_press1.pressing {
                                        "Pressing..."
                                    } else {
                                        "Not pressing"
                                    }}
                                </span>
                            </div>
                            <div class="text-sm text-gray-600">
                                {"Hold for "}{duration}{"ms"}
                            </div>
                            <div class="text-sm text-gray-500 italic">
                                {"Movement allowed during press"}
                            </div>
                        </div>
                    </div>
                </div>


                <div class="bg-white rounded-xl shadow-lg p-6 space-y-6">
                    <h3 class="text-xl font-bold text-gray-800 mb-4 text-center">
                        {"With Move Threshold"}
                    </h3>

                    <div class="text-center space-y-4">
                        <Button
                            button_ref={button2_ref.clone()}
                            class="w-full"
                            onclick={Callback::from(|_| {})}
                        >
                            {"Hold without moving >"}{move_threshold as i32}{"px"}
                        </Button>

                        <div class="space-y-2">
                            <div class="text-lg font-medium text-gray-800">
                                {"Status: "}
                                <span class="font-bold">
                                    {if *long_press2.long_pressed {
                                        "Long pressed!"
                                    } else if *long_press2.pressing {
                                        "Pressing..."
                                    } else {
                                        "Not pressing"
                                    }}
                                </span>
                            </div>
                            <div class="text-sm text-gray-600">
                                {"Hold for "}{duration}{"ms"}
                            </div>
                            <div class="text-sm text-gray-500 italic">
                                {"Max movement: "}{move_threshold as i32}{"px"}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
