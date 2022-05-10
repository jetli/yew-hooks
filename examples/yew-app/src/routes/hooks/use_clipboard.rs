use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_clipboard` demo
#[function_component(UseClipboard)]
pub fn update() -> Html {
    let clipboard = use_clipboard();
    let logo = use_async_with_options(
        async move {
            if let Ok(response) = reqwest::get(
                "https://raw.githubusercontent.com/yewstack/yew/master/website/static/img/logo.png",
            )
            .await
            {
                if let Ok(bytes) = response.bytes().await {
                    Ok(bytes.to_vec())
                } else {
                    Err("Bytes error")
                }
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
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ "Write text to clipboard" }</button>
                    <button onclick={onclick_read_text}>{ "Read text from clipboard" }</button>
                    <button onclick={onclick_write_bytes}>{ "Write bytes(Yew logo bellow) to clipboard" }</button>
                    <button onclick={onclick_read_bytes}>{ "Read bytes from clipboard" }</button>
                    <p>{ format!("Current text: {:?}", *clipboard.text) }</p>
                    <p>{ format!("Copied: {:?}", *clipboard.copied) }</p>
                    <p>{ format!("Is supported: {:?}", *clipboard.is_supported) }</p>
                    <p>{ format!("Current bytes: {:?}", *clipboard.bytes) }</p>
                    <p>{ format!("Current bytes mime type: {:?}", *clipboard.bytes_mime_type) }</p>
                    <p><img src="https://raw.githubusercontent.com/yewstack/yew/master/website/static/img/logo.png"/></p>
                </div>
            </header>
        </div>
    }
}
