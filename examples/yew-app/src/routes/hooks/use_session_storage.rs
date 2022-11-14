use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_session_storage` demo
#[function_component(UseSessionStorage)]
pub fn session_storage() -> Html {
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
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onclick}>{ "Set to bar" }</button>
                    <button onclick={ondelete}>{ "Delete" }</button>
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
