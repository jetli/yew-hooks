use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_raf` demo
#[function_component(UseRaf)]
pub fn raf() -> Html {
    let elapsed = use_raf(5000, 1000);

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ "Elapsed: " }</b><br/>
                        { elapsed }
                    </p>
                </div>
            </header>
        </div>
    }
}
