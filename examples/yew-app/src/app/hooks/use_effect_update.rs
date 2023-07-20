use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_effect_update` demo
#[function_component]
pub fn UseEffectUpdate() -> Html {
    let count = use_state(|| 0);
    let count_effect = use_state(|| 0);
    let count_effect_update = use_state(|| 0);

    {
        let count_effect = count_effect.clone();
        let count = count.clone();
        use_effect_with_deps(
            move |_| {
                count_effect.set(*count_effect + 1);
                || ()
            },
            count,
        );
    }

    {
        let count_effect_update = count_effect_update.clone();
        let count = count.clone();
        // Count for use_effect_update_with_deps is less than use_effect_with_deps by 1.
        use_effect_update_with_deps(
            move |_| {
                count_effect_update.set(*count_effect_update + 1);
                || ()
            },
            count,
        );
    }

    let onclick = Callback::from(move |_| {
        count.set(*count + 1);
    });

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ "Update" }</Button>
                    <p>
                        <b>{ " Count effect: " }</b>
                        { *count_effect }
                        <b>{ " Count effect update: " }</b>
                        { *count_effect_update }
                    </p>
                </div>
            </header>
        </div>
    }
}
