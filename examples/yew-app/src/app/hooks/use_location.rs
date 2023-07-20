use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_location` demo
#[function_component]
pub fn UseLocation() -> Html {
    let location = use_location();

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
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
