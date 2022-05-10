use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_logger` demo
#[function_component(MyComponent)]
fn my_component(props: &MyComponentProps) -> Html {
    // Demo #1, logs whenever anything is updated.
    use_logger("MyComponent Props".to_string(), props.clone());

    let counter = use_counter(0);
    // Demo #2, logs only when `prev_state != next_state`.
    use_logger_eq("MyComponent States".to_string(), counter.clone());

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };

    html! {
        <>
            { "My Component" }
            <p>
                <button onclick={onincrease}>{ "Increase internal" }</button><br/>
                <b>{ " counter: " }</b> { props.counter }
                <b>{ " internal counter: " }</b> { *counter }
                <b>{ " title: " }</b> { &props.title }
            </p>
        </>
    }
}

#[derive(Debug, Properties, PartialEq, Clone)]
struct MyComponentProps {
    pub counter: i32,
    pub title: String,
}

#[function_component(UseLogger)]
pub fn logger() -> Html {
    let toggle = use_toggle("Mount", "Unmount");
    let counter = use_counter(0);

    let onclick = {
        let toggle = toggle.clone();
        Callback::from(move |_| toggle.toggle())
    };

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <button {onclick}>{ *toggle }</button>
                    <button onclick={onincrease}>{ "Increase" }</button>
                    <p>
                        {
                            if *toggle == "Unmount" {
                                html! { <MyComponent counter={*counter} title={"my component".to_string()} /> }
                            } else {
                                html! {}
                            }
                        }
                    </p>
                    <p>
                        <b>{ "Please open the browser console to view the output!" }</b>
                    </p>
                </div>
            </header>
        </div>
    }
}
