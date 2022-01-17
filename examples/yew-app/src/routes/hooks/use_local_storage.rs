use yew::prelude::*;

use yew_hooks::use_local_storage;

/// `use_local_storage` demo
#[function_component(UseLocalStorage)]
pub fn local_storage() -> Html {
    let storage = use_local_storage::<String>("foo".to_string());

    let onclick = {
        let storage = storage.clone();
        Callback::from(move |_| storage.set("bar".to_string()))
    };
    let ondelete = {
        let storage = storage.clone();
        Callback::from(move |_| storage.delete())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onclick}>{ "Set to bar" }</button>
                    <button onclick={ondelete}>{ "Delete" }</button>
                    <p>
                        <b>{ "Current value: " }</b>
                        {
                            if let Some(value) = &*storage {
                                html! { value }
                            } else {
                                html! {}
                            }
                        }
                    </p>
                </div>
            </header>
        </div>
    }
}
