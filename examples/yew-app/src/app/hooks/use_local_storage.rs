use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    name: String,
    token: String,
}

/// `use_local_storage` demo
#[function_component]
pub fn UseLocalStorage() -> Html {
    let storage = use_local_storage::<User>("user".to_string());

    let onclick = {
        let storage = storage.clone();
        Callback::from(move |_| {
            storage.set(User {
                name: String::from("Jet Li"),
                token: String::from("jwt_token"),
            });
        })
    };
    let ondelete = {
        let storage = storage.clone();
        Callback::from(move |_| storage.delete())
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    {
                        (*storage).as_ref().map_or_else(|| html! {
                                <>
                                    <Button onclick={onclick}>{ "Sign in" }</Button>
                                </>
                            }, |user| html! {
                                <>
                                    <Button onclick={ondelete}>{ "Logout" }</Button>
                                    <p>
                                        <b>{ "Current user: " }</b>
                                        { &user.name } { " - " } { &user.token }
                                    </p>
                                </>
                        })
                    }
                </div>
            </header>
        </div>
    }
}
