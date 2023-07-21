use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

mod common;

use common::obtain_result;

wasm_bindgen_test_configure!(run_in_browser);

use yew_hooks::use_counter;

#[wasm_bindgen_test]
async fn use_counter_works() {
    #[function_component]
    fn TestComponent() -> Html {
        let counter = use_counter(0);
        if *counter < 5 {
            counter.increase();
        }
        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*counter}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "5");
}
