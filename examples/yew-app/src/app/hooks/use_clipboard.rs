use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_clipboard` demo
#[function_component]
pub fn UseClipboard() -> Html {
    let clipboard = use_clipboard();
    let logo = use_async_with_options(
        async move {
            if let Ok(response) = reqwest::get(
                "https://raw.githubusercontent.com/yewstack/yew/master/website/static/img/logo.png",
            )
            .await
            {
                (response.bytes().await).map_or(Err("Bytes error"), |bytes| Ok(bytes.to_vec()))
            } else {
                Err("Response err")
            }
        },
        UseAsyncOptions::enable_auto(),
    );

    let onclick = {
        let clipboard = clipboard.clone();
        Callback::from(move |_| {
            clipboard.write_text("hello world!".to_owned());
        })
    };

    let onclick_read_text = {
        let clipboard = clipboard.clone();
        Callback::from(move |_| {
            clipboard.read_text();
        })
    };

    let onclick_write_bytes = {
        let clipboard = clipboard.clone();
        Callback::from(move |_| {
            if let Some(bytes) = &logo.data {
                clipboard.write(bytes.clone(), Some("image/png".to_owned()));
            }
        })
    };

    let onclick_read_bytes = {
        let clipboard = clipboard.clone();
        Callback::from(move |_| {
            clipboard.read();
        })
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button {onclick}>{ "Write text to clipboard" }</Button>
                    <Button onclick={onclick_read_text}>{ "Read text from clipboard" }</Button>
                    <Button onclick={onclick_write_bytes}>{ "Write bytes(Yew logo bellow) to clipboard" }</Button>
                    <Button onclick={onclick_read_bytes}>{ "Read bytes from clipboard" }</Button>
                    <p>{ format!("Current text: {:?}", *clipboard.text) }</p>
                    <p>{ format!("Copied: {:?}", *clipboard.copied) }</p>
                    <p>{ format!("Is supported: {:?}", *clipboard.is_supported) }</p>
                    <p>{ format!("Current bytes: {:?}", *clipboard.bytes) }</p>
                    <p>{ format!("Current bytes mime type: {:?}", *clipboard.bytes_mime_type) }</p>
                    <p><img class="mx-auto" src="https://raw.githubusercontent.com/yewstack/yew/master/website/static/img/logo.png"/></p>
                </div>
            </header>
        </div>
    }
}
