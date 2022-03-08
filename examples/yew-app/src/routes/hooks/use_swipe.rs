use yew::prelude::*;

use yew_hooks::{use_swipe, use_swipe_with_options, UseSwipeDirection, UseSwipeOptions};

/// `use_swipe` demo
#[function_component(UseSwipe)]
pub fn swipe() -> Html {
    let node = use_node_ref();
    // Demo #1, detect swipe for a node.
    let state = use_swipe(node.clone());

    // You can depend on direction/swiping etc.
    {
        let state = state.clone();
        use_effect_with_deps(
            move |direction| {
                // Do something based on direction.
                match **direction {
                    UseSwipeDirection::Left => (),
                    UseSwipeDirection::Right => (),
                    UseSwipeDirection::Up => (),
                    UseSwipeDirection::Down => (),
                    _ => (),
                }
                || ()
            },
            state.direction,
        );
    }

    // Demo #2, detect swipe for window with options, or use `use_swipe_with_window`.
    let _ = use_swipe_with_options(
        NodeRef::default(),
        UseSwipeOptions {
            onswipeend: Some(Box::new(move |_e, direction| {
                // Deal with TouchEvent.
                log::debug!("Swipe direction: {:?}", direction);
            })),
            ..Default::default()
        },
    );

    html! {
        <div class="app">
            <header class="app-header">
                <div ref={node} style="background-color: #61dafb; width: 100%; height: 300px;">
                    <p>
                        <b>{ " swiping: " }</b>
                        { *state.swiping }
                        <b>{ " direction: " }</b>
                        { format!("{:?}", *state.direction) }
                        <b>{ " coords_start: " }</b>
                        { format!("{:?}", *state.coords_start) }
                        <b>{ " coords_end: " }</b>
                        { format!("{:?}", *state.coords_end) }
                        <b>{ " length_x: " }</b>
                        { *state.length_x }
                        <b>{ " length_y: " }</b>
                        { *state.length_y }
                    </p>
                    <p>
                        { "Try to swipe inside this area." }
                    </p>
                </div>
            </header>
        </div>
    }
}
