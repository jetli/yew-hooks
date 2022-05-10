use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_location` demo
#[function_component(UseLocation)]
pub fn location() -> Html {
    let location = use_location();

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ "trigger: " }</b>
                        { &location.trigger }
                    </p>
                    <p>
                        <b>{ "state: " }</b>
                        { format!("{:?}", &location.state) }
                    </p>
                    <p>
                        <b>{ "length: " }</b>
                        { &location.length }
                    </p>
                    <p>
                        <b>{ "hash: " }</b>
                        { &location.hash }
                    </p>
                    <p>
                        <b>{ "host: " }</b>
                        { &location.host }
                    </p>
                    <p>
                        <b>{ "hostname: " }</b>
                        { &location.hostname }
                    </p>
                    <p>
                        <b>{ "href: " }</b>
                        { &location.href }
                    </p>
                    <p>
                        <b>{ "origin: " }</b>
                        { &location.origin }
                    </p>
                    <p>
                        <b>{ "pathname: " }</b>
                        { &location.pathname }
                    </p>
                    <p>
                        <b>{ "port: " }</b>
                        { &location.port }
                    </p>
                    <p>
                        <b>{ "protocol: " }</b>
                        { &location.protocol }
                    </p>
                    <p>
                        <b>{ "search: " }</b>
                        { &location.search }
                    </p>
                </div>
            </header>
        </div>
    }
}
