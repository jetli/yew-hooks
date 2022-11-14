use std::{cell::RefCell, rc::Rc};

use gloo::timers::callback::Timeout;
use js_sys::Array;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{BinaryType, MessageEvent, WebSocket};
use yew::prelude::*;

use super::{use_mut_latest, use_state_ptr_eq, use_unmount, UseStatePtrEqHandle};

pub use web_sys::CloseEvent;

/// The current state of the `WebSocket` connection.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseWebSocketReadyState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// Options for `WebSocket`.
#[derive(Default)]
pub struct UseWebSocketOptions {
    /// `WebSocket` connect callback.
    pub onopen: Option<Box<dyn FnMut(Event)>>,
    /// `WebSocket` message callback for text.
    pub onmessage: Option<Box<dyn FnMut(String)>>,
    /// `WebSocket` message callback for binary.
    pub onmessage_bytes: Option<Box<dyn FnMut(Vec<u8>)>>,
    /// `WebSocket` error callback.
    pub onerror: Option<Box<dyn FnMut(Event)>>,
    /// `WebSocket` close callback.
    pub onclose: Option<Box<dyn FnMut(CloseEvent)>>,

    /// Retry times.
    pub reconnect_limit: Option<u32>,
    /// Retry interval(ms).
    pub reconnect_interval: Option<u32>,
    /// Manually starts connection
    pub manual: Option<bool>,
    /// Sub protocols
    pub protocols: Option<Vec<String>>,
}

/// State handle for the [`use_websocket`] hook.
pub struct UseWebSocketHandle {
    /// The current state of the `WebSocket` connection.
    pub ready_state: UseStateHandle<UseWebSocketReadyState>,
    /// Latest text message received from `WebSocket`.
    pub message: UseStatePtrEqHandle<Option<String>>,
    /// Latest binary message received from `WebSocket`.
    pub message_bytes: UseStatePtrEqHandle<Option<Vec<u8>>>,
    /// The `WebSocket` instance.
    pub ws: Rc<RefCell<Option<WebSocket>>>,

    open: Rc<dyn Fn()>,
    close: Rc<dyn Fn()>,
    send: Rc<dyn Fn(String)>,
    send_bytes: Rc<dyn Fn(Vec<u8>)>,
}

impl UseWebSocketHandle {
    /// Connect `WebSocket` manually. If already connected, close the current one and reconnect.
    pub fn open(&self) {
        (self.open)();
    }

    /// Disconnect `WebSocket` manually.
    pub fn close(&self) {
        (self.close)();
    }

    /// Send text message to `WebSocket`.
    pub fn send(&self, data: String) {
        (self.send)(data);
    }

    /// Send binary message to `WebSocket`.
    pub fn send_bytes(&self, data: Vec<u8>) {
        (self.send_bytes)(data);
    }
}

impl Clone for UseWebSocketHandle {
    fn clone(&self) -> Self {
        Self {
            ready_state: self.ready_state.clone(),
            message: self.message.clone(),
            message_bytes: self.message_bytes.clone(),
            ws: self.ws.clone(),

            open: self.open.clone(),
            close: self.close.clone(),
            send: self.send.clone(),
            send_bytes: self.send_bytes.clone(),
        }
    }
}

/// This hook communicates with `WebSocket`.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseWebSocket)]
/// fn web_socket() -> Html {
///     let history = use_list(vec![]);
///
///     let ws = use_websocket("wss://echo.websocket.events/".to_string());
///     let onclick = {
///         let ws = ws.clone();
///         let history = history.clone();
///         Callback::from(move |_| {
///             let message = "Hello, world!".to_string();
///             ws.send(message.clone());
///             history.push(format!("[send]: {}", message));
///         })
///     };
///     {
///         let history = history.clone();
///         let ws = ws.clone();
///         // Receive message by depending on `ws.message`.
///         use_effect_with_deps(
///             move |message| {
///                 if let Some(message) = &**message {
///                     history.push(format!("[recv]: {}", message.clone()));
///                 }
///                 || ()
///             },
///             ws.message,
///         );
///     }
///
///     html! {
///         <>
///             <p>
///                 <button {onclick} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send" }</button>
///             </p>
///             <p>
///                 <b>{ "Message history: " }</b>
///             </p>
///             {
///                 for history.current().iter().map(|message| {
///                     html! {
///                         <p>{ message }</p>
///                     }
///                 })
///             }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_websocket(url: String) -> UseWebSocketHandle {
    use_websocket_with_options(url, UseWebSocketOptions::default())
}

/// This hook communicates with `WebSocket` with options.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseWebSocket)]
/// fn web_socket() -> Html {
///     let history = use_list(vec![]);
///
///     let ws = {
///         let history = history.clone();
///         use_websocket_with_options(
///             "wss://echo.websocket.events/".to_string(),
///             UseWebSocketOptions {
///                 // Receive message by callback `onmessage`.
///                 onmessage: Some(Box::new(move |message| {
///                     history.push(format!("[recv]: {}", message));
///                 })),
///                 manual: Some(true),
///                 ..Default::default()
///             },
///         )
///     };
///     let onclick = {
///         let ws = ws.clone();
///         let history = history.clone();
///         Callback::from(move |_| {
///             let message = "Hello, world!".to_string();
///             ws.send(message.clone());
///             history.push(format!("[send]: {}", message));
///         })
///     };
///     let onopen = {
///         let ws = ws.clone();
///         Callback::from(move |_| {
///             ws.open();
///         })
///     };
///
///     html! {
///         <>
///             <p>
///                 <button onclick={onopen} disabled={*ws.ready_state != UseWebSocketReadyState::Closed}>{ "Connect" }</button>
///                 <button {onclick} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send with options" }</button>
///             </p>
///             <p>
///                 <b>{ "Message history: " }</b>
///             </p>
///             {
///                 for history.current().iter().map(|message| {
///                     html! {
///                         <p>{ message }</p>
///                     }
///                 })
///             }
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_websocket_with_options(url: String, options: UseWebSocketOptions) -> UseWebSocketHandle {
    let ready_state = use_state(|| UseWebSocketReadyState::Closed);
    let message = use_state_ptr_eq(|| None);
    let message_bytes = use_state_ptr_eq(|| None);
    let ws = use_mut_ref(|| None);

    let onopen_ref = use_mut_latest(options.onopen);
    let onmessage_ref = use_mut_latest(options.onmessage);
    let onmessage_bytes_ref = use_mut_latest(options.onmessage_bytes);
    let onerror_ref = use_mut_latest(options.onerror);
    let onclose_ref = use_mut_latest(options.onclose);
    let reconnect_limit = options.reconnect_limit.unwrap_or(3);
    let reconnect_interval = options.reconnect_interval.unwrap_or(3 * 1000);
    let manual = options.manual.unwrap_or(false);
    let protocols = options.protocols;

    let reconnect_times_ref = use_mut_ref(|| 0);
    let reconnect_timer_ref = use_mut_ref(|| None);
    let unmounted_ref = use_mut_ref(|| false);

    let reconnect = use_mut_ref(|| None);
    let connect_ws = use_mut_ref(|| None);

    *reconnect.borrow_mut() = {
        let ws = ws.clone();
        let reconnect_times_ref = reconnect_times_ref.clone();
        let reconnect_timer_ref = reconnect_timer_ref.clone();
        let connect_ws = connect_ws.clone();
        Some(Rc::new(move || {
            if *reconnect_times_ref.borrow() < reconnect_limit
                && ws
                    .borrow()
                    .as_ref()
                    .map_or(false, |ws: &WebSocket| ws.ready_state() != WebSocket::OPEN)
            {
                let connect_ws = connect_ws.clone();
                let reconnect_times_ref = reconnect_times_ref.clone();
                *reconnect_timer_ref.borrow_mut() =
                    Some(Timeout::new(reconnect_interval, move || {
                        let connect_ws = {
                            let connect_ws = connect_ws.borrow();
                            let connect_ws: &Rc<dyn Fn()> = connect_ws.as_ref().unwrap();
                            connect_ws.clone()
                        };
                        connect_ws();
                        *reconnect_times_ref.borrow_mut() += 1;
                    }));
            }
        }) as Rc<dyn Fn()>)
    };

    *connect_ws.borrow_mut() = {
        let ws = ws.clone();
        let ready_state = ready_state.clone();
        let message = message.clone();
        let message_bytes = message_bytes.clone();
        let url = url.clone();
        let reconnect = reconnect.clone();
        let unmounted_ref = unmounted_ref.clone();
        let onopen_ref = onopen_ref.clone();
        let onmessage_ref = onmessage_ref.clone();
        let onmessage_bytes_ref = onmessage_bytes_ref.clone();
        let onerror_ref = onerror_ref.clone();
        let onclose_ref = onclose_ref.clone();
        let reconnect_timer_ref = reconnect_timer_ref.clone();

        Some(Rc::new(move || {
            *reconnect_timer_ref.borrow_mut() = None;

            {
                let web_socket: &mut Option<WebSocket> = &mut ws.borrow_mut();
                if let Some(web_socket) = web_socket {
                    let _ = web_socket.close();
                }
            }

            let web_socket = {
                protocols.as_ref().map_or_else(
                    || WebSocket::new(&url).unwrap_throw(),
                    |protocols| {
                        let array = protocols
                            .iter()
                            .map(|p| JsValue::from(p.clone()))
                            .collect::<Array>();
                        WebSocket::new_with_str_sequence(&url, &JsValue::from(&array))
                            .unwrap_throw()
                    },
                )
            };
            web_socket.set_binary_type(BinaryType::Arraybuffer);
            ready_state.set(UseWebSocketReadyState::Connecting);

            {
                let unmounted_ref = unmounted_ref.clone();
                let ready_state = ready_state.clone();
                let onopen_ref = onopen_ref.clone();
                let onopen_closure = Closure::wrap(Box::new(move |e: Event| {
                    if *unmounted_ref.borrow() {
                        return;
                    }

                    let onopen_ref = onopen_ref.current();
                    let onopen = &mut *onopen_ref.borrow_mut();
                    if let Some(onopen) = onopen {
                        onopen(e);
                    }
                    ready_state.set(UseWebSocketReadyState::Open);
                }) as Box<dyn FnMut(Event)>);
                web_socket.set_onopen(Some(onopen_closure.as_ref().unchecked_ref()));
                // Forget the closure to keep it alive
                onopen_closure.forget();
            }

            {
                let unmounted_ref = unmounted_ref.clone();
                let message_bytes = message_bytes.clone();
                let message = message.clone();
                let onmessage_ref = onmessage_ref.clone();
                let onmessage_bytes_ref = onmessage_bytes_ref.clone();
                let onmessage_closure = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if *unmounted_ref.borrow() {
                        return;
                    }

                    e.data().dyn_into::<js_sys::ArrayBuffer>().map_or_else(
                        |_| {
                            e.data().dyn_into::<js_sys::JsString>().map_or_else(
                                |_| {
                                    unreachable!("message event, received Unknown: {:?}", e.data());
                                },
                                |txt| {
                                    let txt = String::from(&txt);
                                    let onmessage_ref = onmessage_ref.current();
                                    let onmessage = &mut *onmessage_ref.borrow_mut();
                                    if let Some(onmessage) = onmessage {
                                        let txt = txt.clone();
                                        onmessage(txt);
                                    }
                                    message.set(Some(txt));
                                },
                            );
                        },
                        |array_buffer| {
                            let array = js_sys::Uint8Array::new(&array_buffer);
                            let array = array.to_vec();
                            let onmessage_bytes_ref = onmessage_bytes_ref.current();
                            let onmessage_bytes = &mut *onmessage_bytes_ref.borrow_mut();
                            if let Some(onmessage_bytes) = onmessage_bytes {
                                let array = array.clone();
                                onmessage_bytes(array);
                            }
                            message_bytes.set(Some(array));
                        },
                    );
                })
                    as Box<dyn FnMut(MessageEvent)>);
                web_socket.set_onmessage(Some(onmessage_closure.as_ref().unchecked_ref()));
                onmessage_closure.forget();
            }

            {
                let unmounted_ref = unmounted_ref.clone();
                let ready_state = ready_state.clone();
                let onerror_ref = onerror_ref.clone();
                let reconnect = reconnect.clone();
                let onerror_closure = Closure::wrap(Box::new(move |e: Event| {
                    if *unmounted_ref.borrow() {
                        return;
                    }

                    let reconnect: Rc<dyn Fn()> = { reconnect.borrow().as_ref().unwrap().clone() };
                    reconnect();

                    let onerror_ref = onerror_ref.current();
                    let onerror = &mut *onerror_ref.borrow_mut();
                    if let Some(onerror) = onerror {
                        onerror(e);
                    }
                    ready_state.set(UseWebSocketReadyState::Closed);
                }) as Box<dyn FnMut(Event)>);
                web_socket.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
                onerror_closure.forget();
            }

            {
                let unmounted_ref = unmounted_ref.clone();
                let ready_state = ready_state.clone();
                let onclose_ref = onclose_ref.clone();
                let reconnect = reconnect.clone();
                let onclose_closure = Closure::wrap(Box::new(move |e: CloseEvent| {
                    if *unmounted_ref.borrow() {
                        return;
                    }

                    let reconnect: Rc<dyn Fn()> = { reconnect.borrow().as_ref().unwrap().clone() };
                    reconnect();

                    let onclose_ref = onclose_ref.current();
                    let onclose = &mut *onclose_ref.borrow_mut();
                    if let Some(onclose) = onclose {
                        onclose(e);
                    }
                    ready_state.set(UseWebSocketReadyState::Closed);
                })
                    as Box<dyn FnMut(CloseEvent)>);
                web_socket.set_onclose(Some(onclose_closure.as_ref().unchecked_ref()));
                onclose_closure.forget();
            }

            *ws.borrow_mut() = Some(web_socket);
        }) as Rc<dyn Fn()>)
    };

    let send = {
        let ready_state = ready_state.clone();
        let ws = ws.clone();
        Rc::new(move |data: String| {
            if *ready_state == UseWebSocketReadyState::Open {
                let web_socket: &mut Option<WebSocket> = &mut ws.borrow_mut();
                if let Some(web_socket) = web_socket {
                    let _ = web_socket.send_with_str(&data);
                }
            }
        })
    };

    let send_bytes = {
        let ready_state = ready_state.clone();
        let ws = ws.clone();
        Rc::new(move |data: Vec<u8>| {
            if *ready_state == UseWebSocketReadyState::Open {
                let web_socket: &mut Option<WebSocket> = &mut ws.borrow_mut();
                if let Some(web_socket) = web_socket {
                    let _ = web_socket.send_with_u8_array(&data);
                }
            }
        })
    };

    let open = {
        let reconnect_times_ref = reconnect_times_ref.clone();
        let connect_ws = connect_ws.clone();
        Rc::new(move || {
            *reconnect_times_ref.borrow_mut() = 0;
            let connect_ws: Rc<dyn Fn()> = { connect_ws.borrow().as_ref().unwrap().clone() };
            connect_ws();
        })
    };

    let close = {
        let ws = ws.clone();
        Rc::new(move || {
            *reconnect_timer_ref.borrow_mut() = None;
            *reconnect_times_ref.borrow_mut() = reconnect_limit;

            let web_socket: &mut Option<WebSocket> = &mut ws.borrow_mut();
            if let Some(web_socket) = web_socket {
                let _ = web_socket.close();
            }
        })
    };

    {
        let open = open.clone();
        use_effect_with_deps(
            move |(_, manual)| {
                if !*manual {
                    open();
                }

                || ()
            },
            (url, manual),
        );
    }

    {
        let close = close.clone();
        use_unmount(move || {
            *unmounted_ref.borrow_mut() = true;
            close();
        });
    }

    UseWebSocketHandle {
        ready_state,
        message,
        message_bytes,
        ws,
        open,
        close,
        send,
        send_bytes,
    }
}
