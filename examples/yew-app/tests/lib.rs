use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;

wasm_bindgen_test_configure!(run_in_browser);

use yew_app::app::App;

#[wasm_bindgen_test]
async fn app_has_a_home_page() {
    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::ZERO).await;

    let learn_yew = gloo::utils::document()
        .get_element_by_id("yew_hooks")
        .expect("No Yew Hooks or no home page")
        .inner_html();
    assert_eq!(learn_yew, "Yew Hooks");
}
