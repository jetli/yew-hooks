use gloo::dialogs::alert;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_is_mounted` demo
#[function_component]
pub fn UseIsMounted() -> Html {
    let is_mounted = use_is_mounted();

    {
        let is_mounted = is_mounted.clone();
        use_timeout(
            move || {
                alert(format!("Is mounted: {:?}", is_mounted()).as_str());
            },
            2000,
        );
    }

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ "Is mounted: " }</b>
                        { is_mounted() }
                    </p>
                </div>
            </header>
        </div>
    }
}
