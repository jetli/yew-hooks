use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

mod common;

use common::obtain_result;

wasm_bindgen_test_configure!(run_in_browser);

use yew_hooks::prelude::*;

#[wasm_bindgen_test]
async fn use_idle_works() {
    #[function_component]
    fn TestComponent() -> Html {
        let idle = use_idle(Duration::from_millis(100)); // Short timeout for testing

        // Reset idle state on mount to ensure consistent test
        {
            let idle = idle.clone();
            use_effect_once(move || {
                if *idle {
                    idle.reset_idle();
                }
                || {}
            });
        }

        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*idle}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    // Wait a bit for initial render
    sleep(Duration::from_millis(50)).await;

    // Should not be idle yet (within 100ms timeout)
    let result = obtain_result();
    assert_eq!(result.as_str(), "false");

    // Wait longer than the idle timeout
    sleep(Duration::from_millis(150)).await;

    // Should be idle now
    let result = obtain_result();
    assert_eq!(result.as_str(), "true");
}

#[wasm_bindgen_test]
async fn use_idle_reset_works() {
    #[function_component]
    fn TestComponent() -> Html {
        let idle = use_idle(Duration::from_millis(100));

        // Reset idle state after it becomes idle
        {
            let idle_clone = idle.clone();
            use_effect_with(idle.is_idle(), move |is_idle| {
                if *is_idle {
                    idle_clone.reset_idle();
                }
                || {}
            });
        }

        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*idle}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    // Wait for idle timeout
    sleep(Duration::from_millis(150)).await;

    // Should have become idle and then been reset
    let result = obtain_result();
    assert_eq!(result.as_str(), "false");
}

#[wasm_bindgen_test]
async fn use_idle_with_options_works() {
    #[function_component]
    fn TestComponent() -> Html {
        let idle = use_idle_with_options(UseIdleOptions {
            timeout: 100, // 100ms
            initial_idle: true,
            ..Default::default()
        });

        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*idle}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<TestComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    // Should start as idle due to initial_idle: true
    sleep(Duration::from_millis(10)).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "true");
}
