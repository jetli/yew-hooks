use std::rc::Rc;

use gloo::file::Blob as GlooBlob;
use js_sys::{Array, ArrayBuffer, Object, Reflect, Uint8Array};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::Blob;
use yew::prelude::*;

use super::{use_state_ptr_eq, UseStatePtrEqHandle};
use crate::web_sys_ext::{window, ClipboardItem};

/// State handle for the [`use_clipboard`] hook.
pub struct UseClipboardHandle {
    /// The text that is read from or written to clipboard.
    pub text: UseStatePtrEqHandle<Option<String>>,
    /// The bytes that is read from or written to clipboard.
    pub bytes: UseStatePtrEqHandle<Option<Vec<u8>>>,
    /// The mime type of the bytes that is read from or written to clipboard.
    pub bytes_mime_type: UseStatePtrEqHandle<Option<String>>,
    /// If the content is already copied.
    pub copied: UseStatePtrEqHandle<bool>,
    /// If the clipboard is supported.
    pub is_supported: Rc<bool>,

    write_text: Rc<dyn Fn(String)>,
    write: Rc<dyn Fn(Vec<u8>, Option<String>)>,
    read_text: Rc<dyn Fn()>,
    read: Rc<dyn Fn()>,
}

impl UseClipboardHandle {
    /// Read bytes from clipboard.
    pub fn read(&self) {
        (self.read)();
    }

    /// Read text from clipboard.
    pub fn read_text(&self) {
        (self.read_text)();
    }

    /// Write bytes with mime type to clipboard.
    pub fn write(&self, data: Vec<u8>, mime_type: Option<String>) {
        (self.write)(data, mime_type);
    }

    /// Write text to clipboard.
    pub fn write_text(&self, data: String) {
        (self.write_text)(data);
    }
}

impl Clone for UseClipboardHandle {
    fn clone(&self) -> Self {
        Self {
            text: self.text.clone(),
            bytes: self.bytes.clone(),
            bytes_mime_type: self.bytes_mime_type.clone(),
            is_supported: self.is_supported.clone(),
            copied: self.copied.clone(),

            write_text: self.write_text.clone(),
            write: self.write.clone(),
            read_text: self.read_text.clone(),
            read: self.read.clone(),
        }
    }
}

/// This hook is used to read from or write to clipboard for text or bytes.
/// e.g. copy plain text or copy `image/png` file to clipboard.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseClipboard)]
/// fn clipboard() -> Html {
///     let clipboard = use_clipboard();
///
///     let onclick_write_text = {
///         let clipboard = clipboard.clone();
///         Callback::from(move |_| {
///             clipboard.write_text("hello world!".to_owned());
///         })
///     };
///     let onclick_read_text = {
///         let clipboard = clipboard.clone();
///         Callback::from(move |_| {
///             clipboard.read_text();
///         })
///     };
///     let onclick_write_bytes = {
///         let clipboard = clipboard.clone();
///         Callback::from(move |_| {
///             clipboard.write(vec![], Some("image/png".to_owned()));
///         })
///     };
///     let onclick_read_bytes = {
///         let clipboard = clipboard.clone();
///         Callback::from(move |_| {
///             clipboard.read();
///         })
///     };
///     
///     html! {
///         <div>
///             <button onclick={onclick_write_text}>{ "Write text to clipboard" }</button>
///             <button onclick={onclick_read_text}>{ "Read text from clipboard" }</button>
///             <button onclick={onclick_write_bytes}>{ "Write bytes to clipboard" }</button>
///             <button onclick={onclick_read_bytes}>{ "Read bytes from clipboard" }</button>
///             <p>{ format!("Current text: {:?}", *clipboard.text) }</p>
///             <p>{ format!("Copied: {:?}", *clipboard.copied) }</p>
///             <p>{ format!("Is supported: {:?}", *clipboard.is_supported) }</p>
///             <p>{ format!("Current bytes: {:?}", *clipboard.bytes) }</p>
///             <p>{ format!("Current bytes mime type: {:?}", *clipboard.bytes_mime_type) }</p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_clipboard() -> UseClipboardHandle {
    let text = use_state_ptr_eq(|| None);
    let bytes = use_state_ptr_eq(|| None);
    let bytes_mime_type = use_state_ptr_eq(|| None);
    let is_supported = use_memo((), |_| {
        window()
            .expect_throw("Can't find the global Window")
            .navigator()
            .clipboard()
            .is_some()
    });
    let copied = use_state_ptr_eq(|| false);

    let clipboard = use_memo((), |_| {
        window()
            .expect_throw("Can't find the global Window")
            .navigator()
            .clipboard()
    });

    let write_text = {
        let clipboard = clipboard.clone();
        let text = text.clone();
        let copied = copied.clone();
        Rc::new(move |data: String| {
            if let Some(clipboard) = &*clipboard {
                let text = text.clone();
                let text2 = text.clone();
                let copied = copied.clone();
                let copied2 = copied.clone();
                let data2 = data.clone();
                let resolve_closure = Closure::wrap(Box::new(move |_| {
                    text.set(Some(data.clone()));
                    copied.set(true);
                }) as Box<dyn FnMut(JsValue)>);
                let reject_closure = Closure::wrap(Box::new(move |_| {
                    text2.set(None);
                    copied2.set(false);
                }) as Box<dyn FnMut(JsValue)>);
                let _ = clipboard
                    .write_text(&data2)
                    .then2(&resolve_closure, &reject_closure);
                resolve_closure.forget();
                reject_closure.forget();
            }
        })
    };

    let write = {
        let clipboard = clipboard.clone();
        let bytes = bytes.clone();
        let bytes_mime_type = bytes_mime_type.clone();
        let copied = copied.clone();
        Rc::new(move |data: Vec<u8>, mime_type: Option<String>| {
            if let Some(clipboard) = &*clipboard {
                let blob = GlooBlob::new_with_options(&*data, mime_type.as_deref());
                let object = Object::new();
                if Reflect::set(
                    &object,
                    &JsValue::from(mime_type.as_deref()),
                    &JsValue::from(blob),
                )
                .is_ok()
                {
                    if let Ok(item) = ClipboardItem::new(&object) {
                        let items = Array::new();
                        items.push(&item);
                        let bytes = bytes.clone();
                        let bytes2 = bytes.clone();
                        let bytes_mime_type = bytes_mime_type.clone();
                        let bytes_mime_type2 = bytes_mime_type.clone();
                        let copied = copied.clone();
                        let copied2 = copied.clone();
                        let resolve_closure = Closure::wrap(Box::new(move |_| {
                            bytes.set(Some(data.clone()));
                            bytes_mime_type.set(mime_type.clone());
                            copied.set(true);
                        })
                            as Box<dyn FnMut(JsValue)>);
                        let reject_closure = Closure::wrap(Box::new(move |_| {
                            bytes2.set(None);
                            bytes_mime_type2.set(None);
                            copied2.set(false);
                        })
                            as Box<dyn FnMut(JsValue)>);
                        let _ = clipboard
                            .write(&items)
                            .then2(&resolve_closure, &reject_closure);
                        resolve_closure.forget();
                        reject_closure.forget();
                    }
                }
            }
        })
    };

    let read_text = {
        let clipboard = clipboard.clone();
        let text = text.clone();
        Rc::new(move || {
            if let Some(clipboard) = &*clipboard {
                let text = text.clone();
                let text2 = text.clone();
                let resolve_closure = Closure::wrap(Box::new(move |data: JsValue| {
                    data.as_string().map_or_else(
                        || {
                            text.set(None);
                        },
                        |data| {
                            if data.is_empty() {
                                text.set(None);
                            } else {
                                text.set(Some(data));
                            }
                        },
                    );
                }) as Box<dyn FnMut(JsValue)>);
                let reject_closure = Closure::wrap(Box::new(move |_| {
                    text2.set(None);
                }) as Box<dyn FnMut(JsValue)>);
                let _ = clipboard
                    .read_text()
                    .then2(&resolve_closure, &reject_closure);
                resolve_closure.forget();
                reject_closure.forget();
            }
        })
    };

    let read = {
        let bytes = bytes.clone();
        let bytes_mime_type = bytes_mime_type.clone();
        Rc::new(move || {
            if let Some(clipboard) = &*clipboard {
                let bytes = bytes.clone();
                let bytes2 = bytes.clone();
                let bytes_mime_type = bytes_mime_type.clone();
                let bytes_mime_type2 = bytes_mime_type.clone();
                let resolve_closure = Closure::wrap(Box::new(move |items| {
                    let items = Array::from(&items);
                    let bytes = bytes.clone();
                    for item in items.iter() {
                        item.dyn_into::<ClipboardItem>().map_or_else(
                            |_| {
                                bytes.set(None);
                                bytes_mime_type.set(None);
                            },
                            |item| {
                                for t in item.types().iter() {
                                    t.as_string().map_or_else(
                                        || {
                                            bytes.set(None);
                                            bytes_mime_type.set(None);
                                        },
                                        |t| {
                                            let bytes = bytes.clone();
                                            let bytes2 = bytes.clone();
                                            let bytes_mime_type = bytes_mime_type.clone();
                                            let bytes_mime_type2 = bytes_mime_type.clone();
                                            let t2 = t.clone();
                                            let resolve_closure =
                                                Closure::wrap(Box::new(move |blob: JsValue| {
                                                    blob.dyn_into::<Blob>().map_or_else(
                                                        |_| {
                                                            bytes.set(None);
                                                            bytes_mime_type.set(None);
                                                        },
                                                        |blob| {
                                                            let bytes = bytes.clone();
                                                            let bytes2 = bytes.clone();
                                                            let bytes_mime_type =
                                                                bytes_mime_type.clone();
                                                            let bytes_mime_type2 =
                                                                bytes_mime_type.clone();
                                                            let t = t.clone();
                                                            let resolve_closure = Closure::wrap(
                                                                Box::new(move |buffer: JsValue| {
                                                                    buffer
                                                               .dyn_into::<ArrayBuffer>()
                                               .map_or_else(
                                                           |_| {
                                                                       bytes.set(None);
                                                                       bytes_mime_type.set(None);
                                                                   },
                                                           |buffer| {
                                                                       let data = Uint8Array::new(
                                                                                   &buffer,
                                                                               )
                                                                       .to_vec();
                                                                       bytes.set(Some(data));
                                                                       bytes_mime_type
                                                                           .set(Some(t.clone()));
                                                                   },
                                                       );
                                                                })
                                                                    as Box<dyn FnMut(JsValue)>,
                                                            );
                                                            let reject_closure =
                                                                Closure::wrap(Box::new(move |_| {
                                                                    bytes2.set(None);
                                                                    bytes_mime_type2.set(None);
                                                                })
                                                                    as Box<dyn FnMut(JsValue)>);
                                                            let _ = blob.array_buffer().then2(
                                                                &resolve_closure,
                                                                &reject_closure,
                                                            );
                                                            resolve_closure.forget();
                                                            reject_closure.forget();
                                                        },
                                                    );
                                                })
                                                    as Box<dyn FnMut(JsValue)>);
                                            let reject_closure = Closure::wrap(Box::new(move |_| {
                                                bytes2.set(None);
                                                bytes_mime_type2.set(None);
                                            })
                                                as Box<dyn FnMut(JsValue)>);
                                            let _ = item
                                                .get_type(&t2)
                                                .then2(&resolve_closure, &reject_closure);
                                            resolve_closure.forget();
                                            reject_closure.forget();
                                        },
                                    );
                                }
                            },
                        );
                    }
                }) as Box<dyn FnMut(JsValue)>);
                let reject_closure = Closure::wrap(Box::new(move |_| {
                    bytes2.set(None);
                    bytes_mime_type2.set(None);
                }) as Box<dyn FnMut(JsValue)>);
                let _ = clipboard.read().then2(&resolve_closure, &reject_closure);
                resolve_closure.forget();
                reject_closure.forget();
            }
        })
    };

    UseClipboardHandle {
        text,
        bytes,
        bytes_mime_type,
        copied,
        is_supported,
        write_text,
        write,
        read_text,
        read,
    }
}
