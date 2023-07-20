use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_session_storage` demo
#[function_component]
pub fn UseSessionStorage() -> Html {
    let storage = use_session_storage::<String>("foo".to_string());

    let onclick = {
        let storage = storage.clone();
        Callback::from(move |_| storage.set("bar".to_string()))
    };
    let ondelete = {
        let storage = storage.clone();
        Callback::from(move |_| storage.delete())
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onclick}>{ "Set to bar" }</Button>
                    <Button onclick={ondelete}>{ "Delete" }</Button>
                    <p>
                        <b>{ "Current value: " }</b>
                        {
                            (*storage)
                                .as_ref()
                                .map_or_else(|| html! {}, |value| html! { value })
                        }
                    </p>
                </div>
            </header>
        </div>
    }
}
