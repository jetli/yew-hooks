use gloo::dialogs::alert;

use yew::prelude::*;

use yew_hooks::{use_is_mounted, use_timeout};

/// `use_is_mounted` demo
#[function_component(UseIsMounted)]
pub fn is_mounted() -> Html {
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
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ "Is mounted: " }</b>
                        { is_mounted() }
                    </p>
                </div>
            </header>
        </div>
    }
}
