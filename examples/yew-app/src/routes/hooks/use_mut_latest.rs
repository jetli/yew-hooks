use gloo::timers::callback::Interval;
use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_mut_latest` demo
#[function_component(UseMutLatest)]
pub fn mut_latest() -> Html {
    let state = use_state(|| 0);
    let interval = use_mut_ref(|| None);
    let closure = {
        let state = state.clone();
        move || state.set(*state + 1)
    };

    let latest_closure = use_mut_latest(closure);

    let state2 = use_state(|| 0);
    let interval2 = use_mut_ref(|| None);
    let closure2 = {
        let state2 = state2.clone();
        move || state2.set(*state2 + 1)
    };

    use_effect_once(move || {
        *interval.borrow_mut() = Some(Interval::new(1000, move || {
            let latest_closure = latest_closure.current();
            let closure = &*latest_closure.borrow_mut();
            // This will get the latest closure and increase state by 1 each time.
            closure();
        }));
        move || *interval.borrow_mut() = None
    });

    use_effect_once(move || {
        *interval2.borrow_mut() = Some(Interval::new(1000, move || {
            // This will NOT get the latest closure2 but always the initial 0 each time.
            closure2();
        }));
        move || *interval2.borrow_mut() = None
    });

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
