use std::cmp::max;
use std::rc::Rc;

use yew::prelude::*;

use super::{use_event, use_mut_latest};

/// Swipe direction.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum UseSwipeDirection {
    Up,
    Right,
    Down,
    Left,
    None,
}

/// Options for swipe.
#[derive(Default)]
pub struct UseSwipeOptions {
    pub threshold: Option<u32>,
    pub onswipestart: Option<Box<dyn FnMut(TouchEvent)>>,
    pub onswipe: Option<Box<dyn FnMut(TouchEvent)>>,
    pub onswipeend: Option<Box<dyn FnMut(TouchEvent, UseSwipeDirection)>>,
}

/// State handle for the [`use_swipe`] hook.
pub struct UseSwipeHandle {
    pub swiping: UseStateHandle<bool>,
    pub direction: UseStateHandle<UseSwipeDirection>,
    pub coords_start: UseStateHandle<(i32, i32)>,
    pub coords_end: UseStateHandle<(i32, i32)>,
    pub length_x: UseStateHandle<i32>,
    pub length_y: UseStateHandle<i32>,
}

impl Clone for UseSwipeHandle {
    fn clone(&self) -> Self {
        Self {
            swiping: self.swiping.clone(),
            direction: self.direction.clone(),
            coords_start: self.coords_start.clone(),
            coords_end: self.coords_end.clone(),
            length_x: self.length_x.clone(),
            length_y: self.length_y.clone(),
        }
    }
}

/// A sensor hook that detects swipe based on TouchEvent.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseSwipe)]
/// fn swipe() -> Html {
///     let node =  use_node_ref();
///     let state = use_swipe(node.clone());
///
///     // You can depend on direction/swiping etc.
///     {
///         let state = state.clone();
///         use_effect_with_deps(move |direction| {
///             // Do something based on direction.
///             match **direction {
///                 UseSwipeDirection::Left => (),
///                 UseSwipeDirection::Right => (),
///                 UseSwipeDirection::Up => (),
///                 UseSwipeDirection::Down => (),
///                 _ => (),
///             }
///             || ()
///         }, state.direction);
///     }
///     
///     html! {
///         <div ref={node}>
///             <b>{ " swiping: " }</b>
///             { *state.swiping }
///             <b>{ " direction: " }</b>
///             { format!("{:?}", *state.direction) }
///             <b>{ " coords_start: " }</b>
///             { format!("{:?}", *state.coords_start) }
///             <b>{ " coords_end: " }</b>
///             { format!("{:?}", *state.coords_end) }
///             <b>{ " length_x: " }</b>
///             { *state.length_x }
///             <b>{ " length_y: " }</b>
///             { *state.length_y }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_swipe(node: NodeRef) -> UseSwipeHandle {
    use_swipe_with_options(node, UseSwipeOptions::default())
}

/// A sensor hook that detects swipe based on TouchEvent for window.
/// See [`use_swipe`].
#[hook]
pub fn use_swipe_with_window() -> UseSwipeHandle {
    use_swipe_with_options(NodeRef::default(), UseSwipeOptions::default())
}

/// A sensor hook that detects swipe based on TouchEvent with options.
/// If you want to detect for window, pass `NodeRef::default()` to param `node`.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseSwipe)]
/// fn swipe() -> Html {
///     let node =  use_node_ref();
///     let state = use_swipe_with_options(node.clone(), UseSwipeOptions {
///         onswipeend: Some(Box::new(move |_e, direction| {
///             // Deal with TouchEvent.
///             log::debug!("Swipe direction: {:?}", direction);
///         })),
///         ..Default::default()
///     });
///     
///     html! {
///         <div ref={node}>
///             <b>{ " swiping: " }</b>
///             { *state.swiping }
///             <b>{ " direction: " }</b>
///             { format!("{:?}", *state.direction) }
///             <b>{ " coords_start: " }</b>
///             { format!("{:?}", *state.coords_start) }
///             <b>{ " coords_end: " }</b>
///             { format!("{:?}", *state.coords_end) }
///             <b>{ " length_x: " }</b>
///             { *state.length_x }
///             <b>{ " length_y: " }</b>
///             { *state.length_y }
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_swipe_with_options(node: NodeRef, options: UseSwipeOptions) -> UseSwipeHandle {
    let swiping = use_state_eq(|| false);
    let direction = use_state_eq(|| UseSwipeDirection::None);
    let coords_start = use_state(|| (0, 0));
    let coords_end = use_state(|| (0, 0));
    let length_x = use_state(|| 0);
    let length_y = use_state(|| 0);

    let threshold = options.threshold.unwrap_or(50);
    let onswipestart = use_mut_latest(options.onswipestart);
    let onswipe = use_mut_latest(options.onswipe);
    let onswipeend = use_mut_latest(options.onswipeend);

    let diff_x = {
        let coords_start = coords_start.clone();
        let coords_end = coords_end.clone();
        #[allow(clippy::unnecessary_cast)]
        Rc::new(move || (coords_start.0 - coords_end.0) as i32)
    };

    let diff_y = {
        let coords_start = coords_start.clone();
        let coords_end = coords_end.clone();
        #[allow(clippy::unnecessary_cast)]
        Rc::new(move || (coords_start.1 - coords_end.1) as i32)
    };

    let ontouchend = {
        let swiping = swiping.clone();
        let direction = direction.clone();
        Rc::new(move |e: TouchEvent| {
            if *swiping {
                let onswipeend = onswipeend.current();
                let onswipeend = &mut *onswipeend.borrow_mut();
                if let Some(onswipeend) = onswipeend {
                    onswipeend(e, (*direction).clone());
                }
            }

            swiping.set(false);
            direction.set(UseSwipeDirection::None);
        })
    };

    let threshold_exceeded = {
        let diff_x = diff_x.clone();
        let diff_y = diff_y.clone();
        Rc::new(move || max(diff_x().abs(), diff_y().abs()) >= (threshold as i32))
    };

    {
        let node = node.clone();
        let coords_start = coords_start.clone();
        let coords_end = coords_end.clone();
        use_event(node, "touchstart", move |e: TouchEvent| {
            let x = e.touches().get(0).map_or(0, |t| t.client_x());
            let y = e.touches().get(0).map_or(0, |t| t.client_y());
            coords_start.set((x, y));
            coords_end.set((x, y));

            let onswipestart = onswipestart.current();
            let onswipestart = &mut *onswipestart.borrow_mut();
            if let Some(onswipestart) = onswipestart {
                onswipestart(e);
            }
        });
    }

    {
        let node = node.clone();
        let coords_end = coords_end.clone();
        let swiping = swiping.clone();
        let length_x = length_x.clone();
        let length_y = length_y.clone();
        let direction = direction.clone();
        use_event(node, "touchmove", move |e: TouchEvent| {
            let x = e.touches().get(0).map_or(0, |t| t.client_x());
            let y = e.touches().get(0).map_or(0, |t| t.client_y());
            coords_end.set((x, y));
            length_x.set(diff_x());
            length_y.set(diff_y());

            if !*swiping && threshold_exceeded() {
                swiping.set(true);
            }

            if !threshold_exceeded() {
                direction.set(UseSwipeDirection::None);
            } else if diff_x().abs() > diff_y().abs() {
                if diff_x() > 0 {
                    direction.set(UseSwipeDirection::Left);
                } else {
                    direction.set(UseSwipeDirection::Right);
                }
            } else if diff_y() > 0 {
                direction.set(UseSwipeDirection::Up);
            } else {
                direction.set(UseSwipeDirection::Down);
            }

            if *swiping {
                let onswipe = onswipe.current();
                let onswipe = &mut *onswipe.borrow_mut();
                if let Some(onswipe) = onswipe {
                    onswipe(e);
                }
            }
        });
    }

    {
        let node = node.clone();
        let ontouchend = ontouchend.clone();
        use_event(node, "touchend", move |e: TouchEvent| {
            ontouchend(e);
        });
    }

    {
        let ontouchend = ontouchend.clone();
        use_event(node, "touchcancel", move |e: TouchEvent| {
            ontouchend(e);
        });
    }

    UseSwipeHandle {
        swiping,
        direction,
        coords_start,
        coords_end,
        length_x,
        length_y,
    }
}
