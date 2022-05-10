use gloo::timers::callback::Interval;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_latest` demo
#[function_component(UseLatest)]
pub fn latest() -> Html {
    let state = use_state(|| 0);
    let interval = use_mut_ref(|| None);

    let latest_state = use_latest(state.clone());

    let state2 = use_state(|| 0);
    let interval2 = use_mut_ref(|| None);

    {
        let state = state.clone();
        use_effect_once(move || {
            *interval.borrow_mut() = Some(Interval::new(1000, move || {
                // This will get the latest state and increase it by 1 each time.
                state.set(**latest_state.current() + 1);
            }));
            move || *interval.borrow_mut() = None
        });
    }

    {
        let state2 = state2.clone();
        use_effect_once(move || {
            *interval2.borrow_mut() = Some(Interval::new(1000, move || {
                // This will NOT get the latest state2 but always the initial 0 each time.
                state2.set(*state2 + 1);
            }));
            move || *interval2.borrow_mut() = None
        });
    }

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ "Latest value: " }</b>
                        { *state }
                    </p>
                    <p>
                        <b>{ "Stale value: " }</b>
                        { *state2 }
                    </p>
                </div>
            </header>
        </div>
    }
}
