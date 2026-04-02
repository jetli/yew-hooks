use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_start_typing` demo
#[function_component]
pub fn UseStartTyping() -> Html {
    let input_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        use_start_typing(move |_event: KeyboardEvent| {
            // Focus the first input element
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let _ = input.focus();
            }
        });
    }

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-y-8">
                    <div class="max-w-2xl mx-auto p-6 rounded-lg shadow">
                        <p class="mb-4">
                            { "This hook triggers when you type alphanumeric keys (A-Z, 0-9) " }
                            { "anywhere on the page, but ONLY when:" }
                        </p>

                        <ul class="list-disc pl-6 mb-6 text-left">
                            <li class="mb-2">{ "No editable element (input, textarea, or contenteditable) is focused" }</li>
                            <li class="mb-2">{ "The pressed key is alphanumeric (A-Z, 0-9)" }</li>
                            <li class="mb-2">{ "No modifier keys (Ctrl, Alt, Meta) are held" }</li>
                        </ul>

                        <div class="space-y-4 mb-6">
                            <div>
                                <label class="block text-sm font-medium mb-2">{ "Try typing here (will NOT trigger):" }</label>
                                <input
                                    ref={input_ref.clone()}
                                    type="text"
                                    class="w-full p-2 border border-gray-300 rounded"
                                    placeholder="Focus here and type - hook won't trigger"
                                />
                            </div>

                            <div>
                                <label class="block text-sm font-medium mb-2">{ "Or here (will NOT trigger):" }</label>
                                <textarea
                                    class="w-full p-2 border border-gray-300 rounded"
                                    placeholder="Focus here and type - hook won't trigger"
                                    rows="2"
                                    value=""
                                />
                            </div>

                            <div>
                                <label class="block text-sm font-medium mb-2">{ "Or this contenteditable div (will NOT trigger):" }</label>
                                <div
                                    contenteditable="true"
                                    class="w-full p-2 border border-gray-300 rounded min-h-[80px] bg-white"
                                >
                                    { "Click here and type - hook won't trigger" }
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
