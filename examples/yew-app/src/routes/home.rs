use yew::prelude::*;

use yew_hooks::{
    use_bool_toggle, use_is_first_mount, use_is_mounted, use_mount, use_toggle, use_unmount,
};

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    let toggle = use_bool_toggle(true);
    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };
    let onreset = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.reset())
    };
    let onset = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.set(false))
    };

    let toggle2 = use_toggle("Hello", "World");
    let onclick2 = {
        let toggle2 = toggle2.clone();
        Callback::from(move |_| toggle2.toggle())
    };

    let toggle3 = use_toggle(1, 2);
    let onclick3 = {
        let toggle3 = toggle3.clone();
        Callback::from(move |_| toggle3.toggle())
    };

    use_mount(|| {
        log::debug!("Running effect once on mount");
    });

    use_unmount(|| {
        log::debug!("Running clean-up of effect on unmount");
    });

    let is_first = use_is_first_mount();

    let is_mounted = use_is_mounted();

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ "Toggle bool" }</button>
                    <button onclick={onreset}>{ "Toggle reset bool" }</button>
                    <button onclick={onset}>{ "Toggle set bool" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *toggle }
                    </p>
                </div>
                <div>
                    <button onclick={onclick2}>{ "Toggle string" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *toggle2 }
                    </p>
                </div>
                <div>
                    <button onclick={onclick3}>{ "Toggle number" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        { *toggle3 }
                    </p>
                </div>
                <div>
                    <p>
                        <b>{ "Is first mount: " }</b>
                        { is_first }
                    </p>
                </div>
                <div>
                    <p>
                        <b>{ "Is mounted: " }</b>
                        { is_mounted }
                    </p>
                </div>
            </header>
        </div>
    }
}
