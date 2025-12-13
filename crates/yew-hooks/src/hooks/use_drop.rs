use std::rc::Rc;

use web_sys::{DataTransfer, File};
use yew::prelude::*;

use super::{use_event, use_mut_latest};
use crate::web_sys_ext::ClipboardEvent;

/// Options for drop.
#[derive(Default)]
pub struct UseDropOptions {
    /// Callback for file drops.
    #[allow(clippy::type_complexity)]
    pub onfiles: Option<Box<dyn FnMut(Vec<File>, DataTransfer)>>,
    /// Callback for text drops.
    pub ontext: Option<Box<dyn FnMut(String, DataTransfer)>>,
    /// Callback for uri drops.
    pub onuri: Option<Box<dyn FnMut(String, DataTransfer)>>,

    /// Callback for `dragover`.
    pub ondragover: Option<Box<dyn FnMut(DragEvent)>>,
    /// Callback for `dragenter`.
    pub ondragenter: Option<Box<dyn FnMut(DragEvent)>>,
    /// Callback for `dragleave`.
    pub ondragleave: Option<Box<dyn FnMut(DragEvent)>>,
    /// Callback for `dragexit`.
    pub ondragexit: Option<Box<dyn FnMut(DragEvent)>>,
    /// Callback for `drop`.
    pub ondrop: Option<Box<dyn FnMut(DragEvent)>>,
    /// Callback for `paste`.
    pub onpaste: Option<Box<dyn FnMut(ClipboardEvent)>>,
}

/// State handle for the [`use_drop`] hook.
pub struct UseDropHandle {
    /// State for whether is over the drop area.
    pub over: UseStateHandle<bool>,
    /// Latest files dropped.
    pub files: UseStateHandle<Option<Vec<File>>>,
    /// Latest text dropped.
    pub text: UseStateHandle<Option<String>>,
    /// Latest uri dropped.
    pub uri: UseStateHandle<Option<String>>,
}

/// This hook tracks file, link and copy-paste drops.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseDrop)]
/// fn drop() -> Html {
///     let node = use_node_ref();
///     let state = use_drop(node.clone());
///
///     html! {
///         <div ref={node}>
///             <p><b>{ " Files: " }</b></p>
///             {if let Some(files) = &*state.files {
///                 html! {for file in files.iter() {
///                     <p> { file.name() }</p>
///                 } }
///             } else {
///                 html! {}
///             }} 
///             <p><b>{ " Text: " }</b></p>
///             {if let Some(text) = &*state.text {
///                 html! {<p>{ text }</p>}
///             } else {
///                 html! {}
///             }}
///             <p><b>{ " Uri: " }</b></p>
///             {if let Some(uri) = &*state.uri {
///                 html! {<p>{ uri }</p>}
///             } else {
///                 html! {}
///             }}
///             <p>
///                 { "Try to drag & drop or copy & paste something here, e.g. files, links or text" }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_drop(node: NodeRef) -> UseDropHandle {
    use_drop_with_options(node, UseDropOptions::default())
}

/// This hook tracks file, link and copy-paste drops.
/// [`use_drop`] hook with options.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseDrop)]
/// fn drop() -> Html {
///     let node = use_node_ref();
///     let state = use_drop_with_options(node.clone(), UseDropOptions {
///         onfiles: Some(Box::new(move |files, data_transfer| {
///             // Process files or data_transfer
///         })),
///         ..Default::default()
///     });
///
///     html! {
///         <div ref={node}>
///             <p>
///                 { "Try to drag & drop or copy & paste something here, e.g. files, links or text" }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_drop_with_options(node: NodeRef, options: UseDropOptions) -> UseDropHandle {
    let over = use_state(|| false);
    let files = use_state(|| None);
    let text = use_state(|| None);
    let uri = use_state(|| None);

    let onfiles_ref = use_mut_latest(options.onfiles);
    let ontext_ref = use_mut_latest(options.ontext);
    let onuri_ref = use_mut_latest(options.onuri);

    let ondragover_ref = use_mut_latest(options.ondragover);
    let ondragenter_ref = use_mut_latest(options.ondragenter);
    let ondragleave_ref = use_mut_latest(options.ondragleave);
    let ondragexit_ref = use_mut_latest(options.ondragexit);
    let ondrop_ref = use_mut_latest(options.ondrop);
    let onpaste_ref = use_mut_latest(options.onpaste);

    {
        let over = over.clone();
        use_event(node.clone(), "dragover", move |e: DragEvent| {
            e.prevent_default();
            over.set(true);

            let ondragover_ref = ondragover_ref.current();
            let ondragover = &mut *ondragover_ref.borrow_mut();
            if let Some(ondragover) = ondragover {
                ondragover(e);
            }
        });
    }

    {
        let over = over.clone();
        use_event(node.clone(), "dragenter", move |e: DragEvent| {
            e.prevent_default();
            over.set(true);

            let ondragenter_ref = ondragenter_ref.current();
            let ondragenter = &mut *ondragenter_ref.borrow_mut();
            if let Some(ondragenter) = ondragenter {
                ondragenter(e);
            }
        });
    }

    {
        let over = over.clone();
        use_event(node.clone(), "dragleave", move |e: DragEvent| {
            over.set(false);

            let ondragleave_ref = ondragleave_ref.current();
            let ondragleave = &mut *ondragleave_ref.borrow_mut();
            if let Some(ondragleave) = ondragleave {
                ondragleave(e);
            }
        });
    }

    {
        let over = over.clone();
        use_event(node.clone(), "dragexit", move |e: DragEvent| {
            over.set(false);

            let ondragexit_ref = ondragexit_ref.current();
            let ondragexit = &mut *ondragexit_ref.borrow_mut();
            if let Some(ondragexit) = ondragexit {
                ondragexit(e);
            }
        });
    }

    let on_data_transfer = {
        let uri = uri.clone();
        let files = files.clone();
        let text = text.clone();
        let ontext_ref = ontext_ref.clone();
        let onfiles_ref = onfiles_ref.clone();

        Rc::new(move |data_transfer: DataTransfer| {
            if let Ok(uri_data) = data_transfer.get_data("text/uri-list") {
                if !uri_data.is_empty() {
                    let onuri_ref = onuri_ref.current();
                    let onuri = &mut *onuri_ref.borrow_mut();
                    if let Some(onuri) = onuri {
                        let uri_data = uri_data.clone();
                        onuri(uri_data, data_transfer);
                    }
                    uri.set(Some(uri_data));
                    return;
                }
            }

            if let Some(files_list) = data_transfer.files() {
                if files_list.length() > 0 {
                    log::debug!("files_list");
                    let mut files_vec = vec![];
                    for index in 0..files_list.length() {
                        if let Some(file) = files_list.item(index) {
                            files_vec.push(file);
                        }
                    }

                    let onfiles_ref = onfiles_ref.current();
                    let onfiles = &mut *onfiles_ref.borrow_mut();
                    if let Some(onfiles) = onfiles {
                        let files_vec = files_vec.clone();
                        onfiles(files_vec, data_transfer);
                    }
                    files.set(Some(files_vec));
                    return;
                }
            }

            if let Ok(text_data) = data_transfer.get_data("text") {
                let ontext_ref = ontext_ref.current();
                let ontext = &mut *ontext_ref.borrow_mut();
                if let Some(ontext) = ontext {
                    let text_data = text_data.clone();
                    ontext(text_data, data_transfer);
                }
                text.set(Some(text_data));
            }
        })
    };

    {
        let over = over.clone();
        let on_data_transfer = on_data_transfer.clone();
        use_event(node.clone(), "drop", move |e: DragEvent| {
            e.prevent_default();
            over.set(false);

            if let Some(data_transfer) = e.data_transfer() {
                on_data_transfer(data_transfer);
            }

            let ondrop_ref = ondrop_ref.current();
            let ondrop = &mut *ondrop_ref.borrow_mut();
            if let Some(ondrop) = ondrop {
                ondrop(e);
            }
        });
    }

    {
        let on_data_transfer = on_data_transfer.clone();
        use_event(node, "paste", move |e: ClipboardEvent| {
            if let Some(data_transfer) = e.clipboard_data() {
                on_data_transfer(data_transfer);
            }

            let onpaste_ref = onpaste_ref.current();
            let onpaste = &mut *onpaste_ref.borrow_mut();
            if let Some(onpaste) = onpaste {
                onpaste(e);
            }
        });
    }

    UseDropHandle {
        over,
        files,
        text,
        uri,
    }
}
