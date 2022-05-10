use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    name: String,
    token: String,
}

/// `use_local_storage` demo
#[function_component(UseLocalStorage)]
pub fn local_storage() -> Html {
    let storage = use_local_storage::<User>("user".to_string());

    let onclick = {
        let storage = storage.clone();
        Callback::from(move |_| {
            storage.set(User {
                name: String::from("Jet Li"),
                token: String::from("jwt_token"),
            })
        })
    };
    let ondelete = {
        let storage = storage.clone();
        Callback::from(move |_| storage.delete())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    {
                        if let Some(user) = &*storage {
                            html! {
                                <>
                                    <button onclick={ondelete}>{ "Logout" }</button>
                                    <p>
                                        <b>{ "Current user: " }</b>
                                        { &user.name } { " - " } { &user.token }
                                    </p>
                                </>
                                }
                        } else {
                            html! {
                                <>
                                    <button onclick={onclick}>{ "Sign in" }</button>
                                </>
                            }
                        }
                    }
                </div>
            </header>
        </div>
    }
}
