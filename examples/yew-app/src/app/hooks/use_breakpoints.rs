use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_breakpoints` demo
#[function_component]
pub fn UseBreakpoints() -> Html {
    let tailwind_bp = use_breakpoints(BreakpointConfig::tailwind());
    let bootstrap_bp = use_breakpoints(BreakpointConfig::bootstrap());
    let material_bp = use_breakpoints(BreakpointConfig::material());

    let custom_bps = vec![
        ("phone".to_string(), 0),
        ("tablet".to_string(), 640),
        ("laptop".to_string(), 1024),
        ("desktop".to_string(), 1440),
    ];
    let custom_bp = use_breakpoints_with_default(custom_bps, "xs");

    let width = use_state(|| 0u32);
    {
        let width = width.clone();
        use_event_with_window("resize", move |_: Event| {
            let w = gloo::utils::window()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap() as u32;
            width.set(w);
        });
    }
    {
        let width = width.clone();
        use_mount(move || {
            let w = gloo::utils::window()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap() as u32;
            width.set(w);
        });
    }

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-y-6 max-w-4xl mx-auto">
                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <div class="text-left space-y-3">
                            <p>{ "Resize your browser window to see the active breakpoint update in real time." }</p>
                            <p class="text-sm text-gray-500">
                                { "Current window width: " }<span class="font-mono font-bold">{ *width }{ "px" }</span>
                            </p>
                        </div>
                    </div>

                    // Tailwind breakpoints
                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Tailwind Breakpoints" }</h2>
                        <p class="text-sm text-gray-500 mb-3">{ "Current: " }<span class="font-bold text-emerald-600">{ &tailwind_bp }</span></p>
                        <div class="grid grid-cols-4 gap-3 text-sm">
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if tailwind_bp == "sm" { "bg-green-200 dark:bg-green-800 ring-2 ring-green-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if tailwind_bp == "sm" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "sm" }</p>
                                <p>{ "640px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if tailwind_bp == "md" { "bg-green-200 dark:bg-green-800 ring-2 ring-green-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if tailwind_bp == "md" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "md" }</p>
                                <p>{ "768px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if tailwind_bp == "lg" { "bg-green-200 dark:bg-green-800 ring-2 ring-green-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if tailwind_bp == "lg" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "lg" }</p>
                                <p>{ "1024px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if tailwind_bp == "xl" { "bg-green-200 dark:bg-green-800 ring-2 ring-green-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if tailwind_bp == "xl" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "xl" }</p>
                                <p>{ "1280px" }</p>
                            </div>
                        </div>
                    </div>

                    // Bootstrap breakpoints
                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Bootstrap Breakpoints" }</h2>
                        <p class="text-sm text-gray-500 mb-3">{ "Current: " }<span class="font-bold text-blue-600">{ &bootstrap_bp }</span></p>
                        <div class="grid grid-cols-5 gap-3 text-sm">
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if bootstrap_bp == "sm" { "bg-blue-200 dark:bg-blue-800 ring-2 ring-blue-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if bootstrap_bp == "sm" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "sm" }</p>
                                <p>{ "576px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if bootstrap_bp == "md" { "bg-blue-200 dark:bg-blue-800 ring-2 ring-blue-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if bootstrap_bp == "md" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "md" }</p>
                                <p>{ "768px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if bootstrap_bp == "lg" { "bg-blue-200 dark:bg-blue-800 ring-2 ring-blue-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if bootstrap_bp == "lg" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "lg" }</p>
                                <p>{ "992px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if bootstrap_bp == "xl" { "bg-blue-200 dark:bg-blue-800 ring-2 ring-blue-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if bootstrap_bp == "xl" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "xl" }</p>
                                <p>{ "1200px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if bootstrap_bp == "xxl" { "bg-blue-200 dark:bg-blue-800 ring-2 ring-blue-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if bootstrap_bp == "xxl" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "xxl" }</p>
                                <p>{ "1400px" }</p>
                            </div>
                        </div>
                    </div>

                    // Material Design breakpoints
                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Material Design Breakpoints" }</h2>
                        <p class="text-sm text-gray-500 mb-3">{ "Current: " }<span class="font-bold text-purple-600">{ &material_bp }</span></p>
                        <div class="grid grid-cols-4 gap-3 text-sm">
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if material_bp == "sm" { "bg-purple-200 dark:bg-purple-800 ring-2 ring-purple-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if material_bp == "sm" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "sm" }</p>
                                <p>{ "600px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if material_bp == "md" { "bg-purple-200 dark:bg-purple-800 ring-2 ring-purple-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if material_bp == "md" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "md" }</p>
                                <p>{ "960px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if material_bp == "lg" { "bg-purple-200 dark:bg-purple-800 ring-2 ring-purple-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if material_bp == "lg" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "lg" }</p>
                                <p>{ "1280px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if material_bp == "xl" { "bg-purple-200 dark:bg-purple-800 ring-2 ring-purple-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if material_bp == "xl" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "xl" }</p>
                                <p>{ "1920px" }</p>
                            </div>
                        </div>
                    </div>

                    // Custom breakpoints
                    <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
                        <h2 class="text-2xl font-semibold mb-4">{ "Custom Breakpoints" }</h2>
                        <p class="text-sm text-gray-500 mb-3">{ "Current: " }<span class="font-bold text-orange-600">{ &custom_bp }</span></p>
                        <div class="grid grid-cols-5 gap-3 text-sm">
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if custom_bp == "xs" { "bg-orange-200 dark:bg-orange-800 ring-2 ring-orange-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if custom_bp == "xs" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "xs" }</p>
                                <p>{ "< 0px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if custom_bp == "phone" { "bg-orange-200 dark:bg-orange-800 ring-2 ring-orange-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if custom_bp == "phone" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "phone" }</p>
                                <p>{ "0px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if custom_bp == "tablet" { "bg-orange-200 dark:bg-orange-800 ring-2 ring-orange-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if custom_bp == "tablet" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "tablet" }</p>
                                <p>{ "640px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if custom_bp == "laptop" { "bg-orange-200 dark:bg-orange-800 ring-2 ring-orange-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if custom_bp == "laptop" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "laptop" }</p>
                                <p>{ "1024px" }</p>
                            </div>
                            <div class={format!("p-3 rounded transition-all duration-300 {} {}",
                                if custom_bp == "desktop" { "bg-orange-200 dark:bg-orange-800 ring-2 ring-orange-500" } else { "bg-gray-200 dark:bg-gray-700" },
                                if custom_bp == "desktop" { "scale-100 opacity-100" } else { "scale-75 opacity-40" })}>
                                <p class="font-bold">{ "desktop" }</p>
                                <p>{ "1440px" }</p>
                            </div>
                        </div>
                    </div>
                </div>
            </header>
        </div>
    }
}
