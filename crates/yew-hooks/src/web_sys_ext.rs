//! ResizeObserver/ClipboardEvent in web-sys is unstable and
//! requires `--cfg=web_sys_unstable_apis` to be activated,
//! which is inconvenient, so copy the binding code here for now.
#![allow(unused_imports)]
#![allow(clippy::unused_unit)]
use wasm_bindgen::{self, prelude::*};
use web_sys::{DataTransfer, DomRectReadOnly, Element, Event};

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
