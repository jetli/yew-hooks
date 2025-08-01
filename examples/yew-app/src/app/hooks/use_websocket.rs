use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_websocket` demo
#[function_component]
pub fn UseWebSocket() -> Html {
    let history = use_list(vec![]);

    // Demo #1, auto connect to websocket by default.
    let ws = use_websocket("wss://echo.websocket.events/".to_string());
    let onclick = {
        let ws = ws.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, world!".to_string();
            ws.send(message.clone());
            history.push(format!("ws1 [send]: {message}"));
        })
    };
    {
        let history = history.clone();
        let ws = ws.clone();
        // Receive message by depending on `ws.message`.
        use_effect_with(ws.message, move |message| {
            if let Some(message) = &**message {
                history.push(format!("ws1 [recv]: {}", message.clone()));
            }
            || ()
        });
    }

    // Demo #2, send bytes to websocket.
    let onclick_bytes = {
        let ws = ws.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = b"Hello, world!\r\n".to_vec();
            ws.send_bytes(message.clone());
            history.push(format!("ws1 [send]: bytes {message:?}"));
        })
    };
    {
        let history = history.clone();
        let ws = ws.clone();
        // Receive message by depending on `ws.message_bytes`.
        use_effect_with(ws.message_bytes, move |message| {
            if let Some(message) = &**message {
                history.push(format!("ws1 [recv]: bytes {:?}", message.clone()));
            }
            || ()
        });
    }

    // Demo #3, manually connect to websocket with custom options.
    let ws2 = {
        let history = history.clone();
        use_websocket_with_options(
            "wss://echo.websocket.events/".to_string(),
            UseWebSocketOptions {
                // Receive message by callback `onmessage`.
                onmessage: Some(Box::new(move |message| {
                    history.push(format!("ws2 [recv]: {message}"));
                })),
                manual: Some(true),
                ..Default::default()
            },
        )
    };
    let onclick2 = {
        let ws2 = ws2.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, yew_hooks!".to_string();
            ws2.send(message.clone());
            history.push(format!("ws2 [send]: {message}"));
        })
    };
    let onopen = {
        let ws2 = ws2.clone();
        Callback::from(move |_| {
            ws2.open();
        })
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p class="space-x-2">
                        <Button {onclick} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send text" }</Button>
                        <Button onclick={onclick_bytes} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send bytes" }</Button>
                    </p>
                    <p class="space-x-2">
                        <Button onclick={onopen} disabled={*ws2.ready_state != UseWebSocketReadyState::Closed}>{ "Connect" }</Button>
                        <Button onclick={onclick2} disabled={*ws2.ready_state != UseWebSocketReadyState::Open}>{ "Send with options" }</Button>
                    </p>
                    <p>
                        <b>{ "Message history: " }</b>
                    </p>
                    {
                        for history.current().iter().map(|message| {
                            html! {
                                <p>{ message }</p>
                            }
                        })
                    }
                </div>
            </header>
        </div>
    }
}
