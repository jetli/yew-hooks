use yew::prelude::*;

use yew_hooks::{
    use_bool_toggle, use_interval, use_is_first_mount, use_is_mounted, use_mount, use_timeout,
    use_toggle, use_unmount, use_update,
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
    {
        let is_mounted = is_mounted.clone();
        use_timeout(
            move || {
                log::debug!("Is mounted: {:?}", is_mounted());
            },
            2000,
        );
    }

    let timeout_millis = use_state(|| 0);
    let timeout_state = use_state(get_current_time);
    let on_start_timeout = {
        let timeout_millis = timeout_millis.clone();
        Callback::from(move |_| timeout_millis.set(2000))
    };
    {
        let timeout_state = timeout_state.clone();
        use_timeout(
            move || {
                log::debug!("Timeout!");
                timeout_state.set(get_current_time());
            },
            *timeout_millis,
        );
    }

    let interval_millis = use_state(|| 0);
    let interval_state = use_state(get_current_time);
    let on_start_interval = {
        let interval_millis = interval_millis.clone();
        Callback::from(move |_| interval_millis.set(2000))
    };
    {
        let interval_state = interval_state.clone();
        use_interval(
            move || {
                interval_state.set(get_current_time());
                log::debug!("Interval! {:?}", *interval_state);
            },
            *interval_millis,
        );
    }

    let update = use_update();
    let onclick_update = Callback::from(move |_| {
        update();
    });

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
                        { is_mounted() }
                    </p>
                </div>
                <div>
                    <p>
                        <button onclick={on_start_timeout}>{ "Start timeout" }</button>
                        <b>{ "Timeout state: " }</b>
                        { &*timeout_state }
                    </p>
                </div>
                <div>
                    <p>
                        <button onclick={on_start_interval}>{ "Start interval" }</button>
                        <b>{ "Interval state: " }</b>
                        { &*interval_state }
                    </p>
                </div>
                <div>
                    <p>
                        <button onclick={onclick_update}>{ "Update" }</button>
                        <b>{ "Current time: " }</b>
                        { get_current_time() }
                    </p>
                </div>
            </header>
        </div>
    }
}

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}
