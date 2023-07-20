use yew::prelude::*;

/// About page
#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container text-center">
            <header class="space-y-8">
                <p class="mt-24">
                    <a class="text-emerald-800 underline" href="https://github.com/jetli/yew-hooks" target="_blank" rel="noopener noreferrer"
                    >
                        { "Yew Hooks" }
                    </a>
                    { ", hooks for Yew, inspired by streamich/react-use and alibaba/hooks." }
                </p>
                <p>
                    { "Edit " } <code>{ "src/app/about.rs" }</code> { " and save to reload." }
                </p>
            </header>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use wasm_bindgen_test::*;
    use yew::platform::time::sleep;

    wasm_bindgen_test_configure!(run_in_browser);

    use super::About;

    #[wasm_bindgen_test]
    async fn about_page_has_an_app_link() {
        yew::Renderer::<About>::with_root(
            gloo::utils::document().get_element_by_id("output").unwrap(),
        )
        .render();

        sleep(Duration::ZERO).await;

        let app_links = gloo::utils::document().get_elements_by_tag_name("a");

        assert_eq!(app_links.length(), 1);

        let link = app_links.item(0).expect("No app-link").inner_html();
        assert_eq!(link, "Create Yew App");
    }
}
