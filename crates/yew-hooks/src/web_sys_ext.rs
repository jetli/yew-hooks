//! ResizeObserver/ClipboardEvent in web-sys is unstable and
//! requires `--cfg=web_sys_unstable_apis` to be activated,
//! which is inconvenient, so copy the binding code here for now.
#![allow(unused_imports)]
#![allow(clippy::unused_unit)]
#![allow(clippy::empty_docs)]
use wasm_bindgen::{self, prelude::*};
use web_sys::{DataTransfer, DomRectReadOnly, Element, Event, EventTarget};

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ResizeObserver , typescript_type = "ResizeObserver")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ResizeObserver;

    #[wasm_bindgen(catch, constructor, js_class = "ResizeObserver")]
    pub fn new(callback: &::js_sys::Function) -> Result<ResizeObserver, JsValue>;

    # [wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = disconnect)]
    pub fn disconnect(this: &ResizeObserver);

    # [wasm_bindgen (method , structural , js_class = "ResizeObserver" , js_name = observe)]
    pub fn observe(this: &ResizeObserver, target: &Element);
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ResizeObserverEntry , typescript_type = "ResizeObserverEntry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ResizeObserverEntry;

    # [wasm_bindgen (structural , method , getter , js_class = "ResizeObserverEntry" , js_name = target)]
    pub fn target(this: &ResizeObserverEntry) -> Element;

    # [wasm_bindgen (structural , method , getter , js_class = "ResizeObserverEntry" , js_name = contentRect)]
    pub fn content_rect(this: &ResizeObserverEntry) -> DomRectReadOnly;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Event , extends = :: js_sys :: Object , js_name = ClipboardEvent , typescript_type = "ClipboardEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ClipboardEvent;

    # [wasm_bindgen (structural , method , getter , js_class = "ClipboardEvent" , js_name = clipboardData)]
    pub fn clipboard_data(this: &ClipboardEvent) -> Option<DataTransfer>;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = Clipboard , typescript_type = "Clipboard")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Clipboard;

    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = read)]
    pub fn read(this: &Clipboard) -> ::js_sys::Promise;

    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = readText)]
    pub fn read_text(this: &Clipboard) -> ::js_sys::Promise;

    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = write)]
    pub fn write(this: &Clipboard, data: &::wasm_bindgen::JsValue) -> ::js_sys::Promise;

    # [wasm_bindgen (method , structural , js_class = "Clipboard" , js_name = writeText)]
    pub fn write_text(this: &Clipboard, data: &str) -> ::js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Navigator , typescript_type = "Navigator")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Navigator;

    # [wasm_bindgen (structural , method , getter , js_class = "Navigator" , js_name = clipboard)]
    pub fn clipboard(this: &Navigator) -> Option<Clipboard>;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = Window , typescript_type = "Window")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Window;

    # [wasm_bindgen (structural , method , getter , js_class = "Window" , js_name = navigator)]
    pub fn navigator(this: &Window) -> Navigator;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ClipboardItem , typescript_type = "ClipboardItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ClipboardItem;

    #[wasm_bindgen(catch, constructor, js_class = "ClipboardItem")]
    pub fn new(item: &::js_sys::Object) -> Result<ClipboardItem, JsValue>;

    # [wasm_bindgen (structural , method , getter , js_class = "ClipboardItem" , js_name = types)]
    pub fn types(this: &ClipboardItem) -> ::js_sys::Array;

    # [wasm_bindgen (method , structural , js_class = "ClipboardItem" , js_name = getType)]
    pub fn get_type(this: &ClipboardItem, type_: &str) -> ::js_sys::Promise;
}

pub fn window() -> Option<Window> {
    use wasm_bindgen::JsCast;

    js_sys::global().dyn_into::<Window>().ok()
}
