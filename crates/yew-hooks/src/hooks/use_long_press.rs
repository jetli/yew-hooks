use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::prelude::*;

use super::{use_event, use_event_with_window, use_mut_latest};

/// Options for long press.
#[derive(Default)]
pub struct UseLongPressOptions {
    /// Callback for when long press starts.
    pub onstart: Option<Box<dyn FnMut(MouseEvent)>>,
    /// Callback for when long press ends.
    pub onend: Option<Box<dyn FnMut(MouseEvent)>>,
    /// Callback for when long press is completed (after threshold).
    pub onlongpress: Option<Box<dyn FnMut(MouseEvent)>>,
    /// Duration in milliseconds before long press is triggered.
    pub threshold: Option<u32>,
    /// Maximum movement in pixels allowed during press. If exceeded, long press won't trigger.
    pub move_threshold: Option<f64>,
    /// Whether to prevent default behavior on mouse/touch events.
    pub prevent_default: Option<bool>,
}

/// State handle for the [`use_long_press`] hook.
pub struct UseLongPressHandle {
    /// Whether the element is currently being pressed.
    pub pressing: UseStateHandle<bool>,
    /// Whether the long press threshold has been reached.
    pub long_pressed: UseStateHandle<bool>,
    /// Cancel the current long press timeout.
    cancel: Rc<dyn Fn()>,
}

impl UseLongPressHandle {
    /// Cancel the current long press timeout.
    pub fn cancel(&self) {
        (self.cancel)();
    }
}

impl Clone for UseLongPressHandle {
    fn clone(&self) -> Self {
        Self {
            pressing: self.pressing.clone(),
            long_pressed: self.long_pressed.clone(),
            cancel: self.cancel.clone(),
        }
    }
}

/// A hook that detects when a user presses and holds an element for a specified duration.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseLongPress)]
/// fn long_press() -> Html {
///     let button = use_node_ref();
///
///     let long_press = use_long_press(button.clone(), 1000, move |e: MouseEvent| {
///         debug!("Long pressed for 1 second!");
///     });
///
///     let onmouseup = {
///         let long_press = long_press.clone();
///         Callback::from(move |_| {
///             long_press.cancel();
///         })
///     };
///
///     html! {
///         <>
///             <button
///                 ref={button}
///                 onmouseup={onmouseup}
///             >
///                 { "Press and hold me for 1 second" }
///             </button>
///             <p>
///                 { if *long_press.pressing { "Pressing..." } else { "Not pressing" } }
///             </p>
///             <p>
///                 { if *long_press.long_pressed { "Long pressed!" } else { "Not long pressed yet" } }
///             </p>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_long_press<F>(node: NodeRef, threshold: u32, onlongpress: F) -> UseLongPressHandle
where
    F: FnMut(MouseEvent) + 'static,
{
    let options = UseLongPressOptions {
        threshold: Some(threshold),
        onlongpress: Some(Box::new(onlongpress)),
        ..Default::default()
    };

    use_long_press_with_options(node, options)
}

/// A hook that detects when a user presses and holds an element for a specified duration with options.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseLongPressWithOptions)]
/// fn long_press_with_options() -> Html {
///     let button = use_node_ref();
///
///     let long_press = use_long_press_with_options(
///         button.clone(),
///         UseLongPressOptions {
///             threshold: Some(1000),
///             move_threshold: None,
///             onstart: Some(Box::new(|e: MouseEvent| {
///                 debug!("Press started");
///             })),
///             onend: Some(Box::new(|e: MouseEvent| {
///                 debug!("Press ended");
///             })),
///             onlongpress: Some(Box::new(|e: MouseEvent| {
///                 debug!("Long pressed!");
///             })),
///             prevent_default: Some(true),
///         }
///     );
///
///     html! {
///         <>
///             <button ref={button}>
///                 { "Press and hold me" }
///             </button>
///             <p>
///                 { if *long_press.pressing { "Pressing..." } else { "Not pressing" } }
///             </p>
///         </>
///     }
/// }
/// ```
#[hook]
pub fn use_long_press_with_options(
    node: NodeRef,
    options: UseLongPressOptions,
) -> UseLongPressHandle {
    let pressing = use_state(|| false);
    let long_pressed = use_state(|| false);
    let timeout_ref = use_mut_ref(|| None::<Timeout>);

    let onstart_ref = use_mut_latest(options.onstart);
    let onend_ref = use_mut_latest(options.onend);
    let onlongpress_ref = use_mut_latest(options.onlongpress);
    let threshold = options.threshold.unwrap_or(500);
    let move_threshold = options.move_threshold;
    let prevent_default = options.prevent_default.unwrap_or(true);

    let start_coords = use_mut_ref(|| (0.0, 0.0));

    let cancel = {
        let timeout_ref = timeout_ref.clone();
        let pressing = pressing.clone();
        let long_pressed = long_pressed.clone();
        // onend_ref is used in the closure below

        Rc::new(move || {
            // Clear any existing timeout
            *timeout_ref.borrow_mut() = None;

            // Reset states
            if *pressing {
                pressing.set(false);
            }
            if *long_pressed {
                long_pressed.set(false);
            }
        })
    };

    // Handle mousedown/touchstart
    {
        let pressing = pressing.clone();
        let long_pressed = long_pressed.clone();
        let timeout_ref = timeout_ref.clone();
        let onstart_ref = onstart_ref.clone();
        let onlongpress_ref = onlongpress_ref.clone();
        // cancel is used in the closure below

        let start_coords_clone = start_coords.clone();
        use_event(node.clone(), "mousedown", move |e: MouseEvent| {
            if prevent_default {
                e.prevent_default();
            }

            // Reset states
            pressing.set(true);
            long_pressed.set(false);

            // Store starting coordinates for move threshold check
            *start_coords_clone.borrow_mut() = (e.client_x() as f64, e.client_y() as f64);

            // Call onstart callback
            let onstart_ref = onstart_ref.current();
            let onstart = &mut *onstart_ref.borrow_mut();
            if let Some(onstart) = onstart {
                onstart(e.clone());
            }

            // Clear any existing timeout
            *timeout_ref.borrow_mut() = None;

            // Set new timeout for long press
            let e_clone = e.clone();
            let long_pressed_clone = long_pressed.clone();
            let onlongpress_ref_clone = onlongpress_ref.clone();
            let start_coords_clone2 = start_coords_clone.clone();
            let move_threshold = move_threshold;
            let timeout = Timeout::new(threshold, move || {
                // Check if movement exceeded threshold
                if let Some(move_threshold) = move_threshold {
                    let (start_x, start_y) = *start_coords_clone2.borrow();
                    let current_x = e_clone.client_x() as f64;
                    let current_y = e_clone.client_y() as f64;
                    let distance =
                        ((current_x - start_x).powi(2) + (current_y - start_y).powi(2)).sqrt();

                    if distance > move_threshold {
                        return; // Don't trigger long press if moved too far
                    }
                }

                long_pressed_clone.set(true);

                // Call onlongpress callback
                let onlongpress_ref = onlongpress_ref_clone.current();
                let onlongpress = &mut *onlongpress_ref.borrow_mut();
                if let Some(onlongpress) = onlongpress {
                    onlongpress(e_clone);
                }
            });

            *timeout_ref.borrow_mut() = Some(timeout);
        });
    }

    // Handle touchstart for mobile
    {
        let pressing = pressing.clone();
        let long_pressed = long_pressed.clone();
        let timeout_ref = timeout_ref.clone();
        let onstart_ref = onstart_ref.clone();
        let onlongpress_ref = onlongpress_ref.clone();
        // cancel is used in the closure below

        let start_coords_clone3 = start_coords.clone();
        use_event(node.clone(), "touchstart", move |e: TouchEvent| {
            if prevent_default {
                e.prevent_default();
            }

            // Convert TouchEvent to MouseEvent for consistency
            let mouse_event = MouseEvent::new("mousedown").unwrap();

            // Reset states
            pressing.set(true);
            long_pressed.set(false);

            // Store starting coordinates for move threshold check
            if let Some(touch) = e.touches().get(0) {
                *start_coords_clone3.borrow_mut() =
                    (touch.client_x() as f64, touch.client_y() as f64);
            }

            // Call onstart callback
            let onstart_ref = onstart_ref.current();
            let onstart = &mut *onstart_ref.borrow_mut();
            if let Some(onstart) = onstart {
                onstart(mouse_event.clone());
            }

            // Clear any existing timeout
            *timeout_ref.borrow_mut() = None;

            // Set new timeout for long press
            let mouse_event_clone = mouse_event.clone();
            let long_pressed_clone = long_pressed.clone();
            let onlongpress_ref_clone = onlongpress_ref.clone();
            let start_coords_clone4 = start_coords_clone3.clone();
            let move_threshold = move_threshold;
            let timeout = Timeout::new(threshold, move || {
                // Check if movement exceeded threshold
                if let Some(move_threshold) = move_threshold {
                    let (start_x, start_y) = *start_coords_clone4.borrow();
                    // Get current touch position
                    let current_x = e
                        .changed_touches()
                        .get(0)
                        .map_or(start_x, |t| t.client_x() as f64);
                    let current_y = e
                        .changed_touches()
                        .get(0)
                        .map_or(start_y, |t| t.client_y() as f64);
                    let distance =
                        ((current_x - start_x).powi(2) + (current_y - start_y).powi(2)).sqrt();

                    if distance > move_threshold {
                        return; // Don't trigger long press if moved too far
                    }
                }

                long_pressed_clone.set(true);

                // Call onlongpress callback
                let onlongpress_ref = onlongpress_ref_clone.current();
                let onlongpress = &mut *onlongpress_ref.borrow_mut();
                if let Some(onlongpress) = onlongpress {
                    onlongpress(mouse_event_clone);
                }
            });

            *timeout_ref.borrow_mut() = Some(timeout);
        });
    }

    // Handle mouseup/touchend
    {
        let pressing = pressing.clone();
        let long_pressed = long_pressed.clone();
        let timeout_ref = timeout_ref.clone();
        let onend_ref = onend_ref.clone();
        // cancel is used in the closure below

        let pressing_clone = pressing.clone();
        let timeout_ref_clone = timeout_ref.clone();
        let long_pressed_clone = long_pressed.clone();
        let start_coords_clone5 = start_coords.clone();
        use_event_with_window("mousemove", move |e: MouseEvent| {
            if *pressing_clone && move_threshold.is_some() {
                let (start_x, start_y) = *start_coords_clone5.borrow();
                let current_x = e.client_x() as f64;
                let current_y = e.client_y() as f64;
                let distance =
                    ((current_x - start_x).powi(2) + (current_y - start_y).powi(2)).sqrt();

                if let Some(move_threshold) = move_threshold {
                    if distance > move_threshold {
                        // Cancel the long press if movement exceeds threshold
                        *timeout_ref_clone.borrow_mut() = None;
                        pressing_clone.set(false);
                        long_pressed_clone.set(false);
                    }
                }
            }
        });

        let pressing_clone2 = pressing.clone();
        let long_pressed_clone2 = long_pressed.clone();
        let timeout_ref_clone2 = timeout_ref.clone();
        let onend_ref_clone = onend_ref.clone();
        use_event_with_window("mouseup", move |e: MouseEvent| {
            if *pressing_clone2 {
                // Clear timeout
                *timeout_ref_clone2.borrow_mut() = None;

                // Update states
                pressing_clone2.set(false);

                // Call onend callback if not long pressed
                if !*long_pressed_clone2 {
                    let onend_ref = onend_ref_clone.current();
                    let onend = &mut *onend_ref.borrow_mut();
                    if let Some(onend) = onend {
                        onend(e);
                    }
                }

                // Reset long pressed state
                long_pressed_clone2.set(false);
            }
        });
    }

    // Handle touchend for mobile
    {
        let pressing = pressing.clone();
        let long_pressed = long_pressed.clone();
        let timeout_ref = timeout_ref.clone();
        let onend_ref = onend_ref.clone();
        // cancel is used in the closure below

        let pressing_clone3 = pressing.clone();
        let timeout_ref_clone3 = timeout_ref.clone();
        let long_pressed_clone3 = long_pressed.clone();
        let start_coords_clone6 = start_coords.clone();
        use_event_with_window("touchmove", move |e: TouchEvent| {
            if *pressing_clone3 && move_threshold.is_some() {
                let (start_x, start_y) = *start_coords_clone6.borrow();
                if let Some(touch) = e.touches().get(0) {
                    let current_x = touch.client_x() as f64;
                    let current_y = touch.client_y() as f64;
                    let distance =
                        ((current_x - start_x).powi(2) + (current_y - start_y).powi(2)).sqrt();

                    if let Some(move_threshold) = move_threshold {
                        if distance > move_threshold {
                            // Cancel the long press if movement exceeds threshold
                            *timeout_ref_clone3.borrow_mut() = None;
                            pressing_clone3.set(false);
                            long_pressed_clone3.set(false);
                        }
                    }
                }
            }
        });

        let pressing_clone4 = pressing.clone();
        let long_pressed_clone4 = long_pressed.clone();
        let timeout_ref_clone4 = timeout_ref.clone();
        let onend_ref_clone2 = onend_ref.clone();
        use_event_with_window("touchend", move |_e: TouchEvent| {
            if *pressing_clone4 {
                // Convert TouchEvent to MouseEvent for consistency
                let mouse_event = MouseEvent::new("mouseup").unwrap();

                // Clear timeout
                *timeout_ref_clone4.borrow_mut() = None;

                // Update states
                pressing_clone4.set(false);

                // Call onend callback if not long pressed
                if !*long_pressed_clone4 {
                    let onend_ref = onend_ref_clone2.current();
                    let onend = &mut *onend_ref.borrow_mut();
                    if let Some(onend) = onend {
                        onend(mouse_event);
                    }
                }

                // Reset long pressed state
                long_pressed_clone4.set(false);
            }
        });
    }

    // Handle mouseleave/touchcancel
    {
        let cancel = cancel.clone();

        use_event(node.clone(), "mouseleave", move |_: MouseEvent| {
            cancel();
        });
    }

    {
        let cancel = cancel.clone();

        use_event(node.clone(), "touchcancel", move |_: TouchEvent| {
            cancel();
        });
    }

    // Cleanup on unmount
    {
        let cancel = cancel.clone();
        use_effect_with((), move |_| {
            move || {
                cancel();
            }
        });
    }

    UseLongPressHandle {
        pressing,
        long_pressed,
        cancel,
    }
}
